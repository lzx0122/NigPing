use dashmap::DashMap;
use hickory_resolver::TokioAsyncResolver;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{watch, Mutex, Semaphore};
use tokio::time;

use windows::Win32::NetworkManagement::IpHelper::{
    IcmpCloseHandle, IcmpCreateFile, IcmpSendEcho, ICMP_ECHO_REPLY, IP_OPTION_INFORMATION,
};

const IP_SUCCESS: u32 = 0;
const IP_TTL_EXPIRED_TRANSIT: u32 = 11013;
const DISCOVERY_TIMEOUT_MS: u32 = 1500;
const WORKER_TIMEOUT_MS: u32 = 1000;
const MAX_HOPS: u8 = 40;
const PROBE_INTERVAL_SECS: u64 = 1;
const MAX_CONCURRENT_PINGS: usize = 8;
const ICMP_PAYLOAD_SIZE: usize = 32;
const ICMP_ERROR_BUFFER_SIZE: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HopStat {
    pub hop: u8,
    pub ip: String,
    pub hostname: Option<String>,
    pub loss_pct: f64,
    pub sent: u32,
    pub recv: u32,
    pub last_ms: f64,
    pub avg_ms: f64,
    pub best_ms: f64,
    pub worst_ms: f64,
}

impl HopStat {
    pub fn new(hop: u8, ip: String) -> Self {
        Self {
            hop,
            ip,
            hostname: None,
            loss_pct: 0.0,
            sent: 0,
            recv: 0,
            last_ms: 0.0,
            avg_ms: 0.0,
            best_ms: 0.0,
            worst_ms: 0.0,
        }
    }

    pub fn update(&mut self, is_success: bool, rtt_ms: f64) {
        self.sent += 1;
        if is_success {
            self.recv += 1;
            self.last_ms = rtt_ms;
            if self.recv == 1 {
                self.best_ms = rtt_ms;
                self.worst_ms = rtt_ms;
                self.avg_ms = rtt_ms;
            } else {
                if rtt_ms < self.best_ms {
                    self.best_ms = rtt_ms;
                }
                if rtt_ms > self.worst_ms {
                    self.worst_ms = rtt_ms;
                }
                self.avg_ms = self.avg_ms + (rtt_ms - self.avg_ms) / (self.recv as f64);
            }
        }
        self.loss_pct = ((self.sent - self.recv) as f64 / self.sent as f64) * 100.0;
    }
}

pub struct HopProbeState {
    pub is_running: bool,
    pub target: Option<String>,
    pub cancel_token: Option<watch::Sender<bool>>,
    pub hops: Arc<DashMap<u8, HopStat>>,
    pub session_epoch: Arc<AtomicU64>,
}

impl Default for HopProbeState {
    fn default() -> Self {
        Self {
            is_running: false,
            target: None,
            cancel_token: None,
            hops: Arc::new(DashMap::new()),
            session_epoch: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl HopProbeState {
    pub fn new() -> Self {
        Self::default()
    }
}

fn sync_ping(
    target_ip: Ipv4Addr,
    ttl: u8,
    timeout_ms: u32,
) -> Result<Option<(Ipv4Addr, u32)>, String> {
    sync_ping_icmp(target_ip, ttl, timeout_ms)
}

fn sync_ping_icmp(
    target_ip: Ipv4Addr,
    ttl: u8,
    timeout_ms: u32,
) -> Result<Option<(Ipv4Addr, u32)>, String> {
    unsafe {
        let handle = match IcmpCreateFile() {
            Ok(h) => h,
            Err(_) => return Err("Failed to create ICMP handle".to_string()),
        };

        let buffer_size = std::mem::size_of::<ICMP_ECHO_REPLY>() 
                        + ICMP_PAYLOAD_SIZE 
                        + ICMP_ERROR_BUFFER_SIZE;

        let mut reply_buffer = vec![0u8; buffer_size];

        let ip_options = IP_OPTION_INFORMATION {
            Ttl: ttl,
            Tos: 0,
            Flags: 0,
            OptionsSize: 0,
            OptionsData: std::ptr::null_mut(),
        };

        let request_data = b"PingPal traceroute";

        let reply_count = IcmpSendEcho(
            handle,
            u32::from_ne_bytes(target_ip.octets()),
            request_data.as_ptr() as *const _,
            request_data.len() as u16,
            Some(&ip_options as *const _),
            reply_buffer.as_mut_ptr() as *mut _,
            reply_buffer.len() as u32,
            timeout_ms,
        );

        let _ = IcmpCloseHandle(handle);

        if reply_count > 0 {
            let reply = std::ptr::read_unaligned(
                reply_buffer.as_ptr() as *const ICMP_ECHO_REPLY
            );
            let ip_bytes = reply.Address.to_ne_bytes();
            let replying_ip = Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
            let status = reply.Status;

            if status == IP_SUCCESS || status == IP_TTL_EXPIRED_TRANSIT {
                return Ok(Some((replying_ip, reply.RoundTripTime)));
            }
        }

        Ok(None)
    }
}

async fn resolve_target(target: &str) -> Option<Ipv4Addr> {
    if let Ok(ip) = target.parse::<Ipv4Addr>() {
        return Some(ip);
    }
    if let Ok(resolver) = TokioAsyncResolver::tokio_from_system_conf() {
        if let Ok(response) = resolver.ipv4_lookup(target).await {
            return response.into_iter().next().map(|a| a.0);
        }
    }
    None
}

async fn ping_worker(
    hop_num: u8,
    target_ip: Ipv4Addr,
    ttl: u8,
    hops_map: Arc<DashMap<u8, HopStat>>,
    ping_limiter: Arc<Semaphore>,
    session_epoch: Arc<AtomicU64>,
    session_id: u64,
    mut cancel_rx: watch::Receiver<bool>,
) {
    let mut interval = time::interval(Duration::from_secs(PROBE_INTERVAL_SECS));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if !is_session_active(&session_epoch, session_id, &cancel_rx) {
                    break;
                }
                
                let result = run_sync_ping_limited(target_ip, ttl, WORKER_TIMEOUT_MS, ping_limiter.clone()).await;
                
                let (success, rtt) = match result {
                    Ok(Some((_reply_ip, rtt))) => {
                        (true, rtt as f64)
                    }
                    Err(e) => {
                        eprintln!("[PingPal Probe Worker] Failed at hop {}: {}", hop_num, e);
                        (false, 0.0)
                    }
                    _ => (false, 0.0),
                };

                if !is_session_active(&session_epoch, session_id, &cancel_rx) {
                    break;
                }

                if let Some(mut stat) = hops_map.get_mut(&hop_num) {
                    stat.update(success, rtt);
                }
            }
            _ = cancel_rx.changed() => {
                if !is_session_active(&session_epoch, session_id, &cancel_rx) {
                    break;
                }
            }
        }
    }
}

fn is_session_active(
    session_epoch: &AtomicU64,
    session_id: u64,
    cancel_rx: &watch::Receiver<bool>,
) -> bool {
    session_epoch.load(Ordering::SeqCst) == session_id && !*cancel_rx.borrow()
}

async fn run_sync_ping_limited(
    target_ip: Ipv4Addr,
    ttl: u8,
    timeout_ms: u32,
    ping_limiter: Arc<Semaphore>,
) -> Result<Option<(Ipv4Addr, u32)>, String> {
    let _permit = ping_limiter
        .acquire_owned()
        .await
        .map_err(|_| "Ping limiter is closed".to_string())?;

    tokio::task::spawn_blocking(move || sync_ping(target_ip, ttl, timeout_ms))
        .await
        .map_err(|e| format!("Ping task join error: {}", e))?
}

async fn traceroute_session(
    target_ip: Ipv4Addr,
    hops_map: Arc<DashMap<u8, HopStat>>,
    ping_limiter: Arc<Semaphore>,
    session_epoch: Arc<AtomicU64>,
    session_id: u64,
    cancel_rx: watch::Receiver<bool>,
) {
    for ttl in 1..=MAX_HOPS {
        if !is_session_active(&session_epoch, session_id, &cancel_rx) {
            break;
        }

        let result =
            run_sync_ping_limited(target_ip, ttl, DISCOVERY_TIMEOUT_MS, ping_limiter.clone()).await;

        let (found_ip, found_rtt, is_target) = match result {
            Ok(Some((replying_ip, rtt))) => {
                let is_target = replying_ip == target_ip;
                (Some(replying_ip), Some(rtt), is_target)
            }
            Err(e) => {
                eprintln!("[PingPal Probe Engine] Failed at hop {}: {}", ttl, e);
                (None, None, false)
            }
            _ => (None, None, false),
        };

        if !is_session_active(&session_epoch, session_id, &cancel_rx) {
            break;
        }

        if let Some(ip) = found_ip {
            let mut stat = HopStat::new(ttl, ip.to_string());
            if let Some(rtt) = found_rtt {
                stat.update(true, rtt as f64);
            }
            hops_map.insert(ttl, stat);

            let worker_cancel_rx = cancel_rx.clone();
            let worker_map = hops_map.clone();
            let worker_epoch = session_epoch.clone();
            let worker_limiter = ping_limiter.clone();
            tokio::spawn(async move {
                ping_worker(
                    ttl,
                    target_ip,
                    ttl,
                    worker_map,
                    worker_limiter,
                    worker_epoch,
                    session_id,
                    worker_cancel_rx,
                )
                .await;
            });

            if is_target {
                break;
            }
        } else {
            let mut stat = HopStat::new(ttl, "*".to_string());
            stat.update(false, 0.0);
            hops_map.insert(ttl, stat);
        }
    }
}

#[tauri::command]
pub async fn start_hop_probe(target: String, state: tauri::State<'_, Arc<Mutex<HopProbeState>>>) -> Result<String, String> {
    let target_ip = match resolve_target(&target).await {
        Some(ip) => ip,
        None => return Err(format!("Could not resolve target: {}", target)),
    };
    
    let state_arc = state.inner().clone();
    let mut s = state_arc.lock().await;

    if s.is_running {
        return Err("A probe is already running".to_string());
    }

    s.is_running = true;
    s.target = Some(target.clone());
    s.hops.clear();
    let session_id: u64 = s.session_epoch.fetch_add(1, Ordering::SeqCst) + 1;

    let (cancel_tx, cancel_rx) = watch::channel(false);
    s.cancel_token = Some(cancel_tx);

    let hops_map = s.hops.clone();
    let ping_limiter = Arc::new(Semaphore::new(MAX_CONCURRENT_PINGS));
    let session_epoch: Arc<AtomicU64> = s.session_epoch.clone();

    tokio::spawn(async move {
        traceroute_session(
            target_ip,
            hops_map,
            ping_limiter,
            session_epoch,
            session_id,
            cancel_rx,
        )
        .await;
    });

    Ok(target_ip.to_string())
}

#[tauri::command]
pub async fn stop_hop_probe(state: tauri::State<'_, Arc<Mutex<HopProbeState>>>) -> Result<String, String> {
    let state_arc = state.inner().clone();
    let mut s = state_arc.lock().await;
    
    if !s.is_running {
        return Err("No active probe".to_string());
    }
    
    if let Some(cancel_tx) = &s.cancel_token {
        let _ = cancel_tx.send(true);
    }

    s.session_epoch.fetch_add(1, Ordering::SeqCst);
    
    s.is_running = false;
    s.cancel_token = None;
    s.target = None;
    Ok("Hop probe stopped".to_string())
}

#[tauri::command]
pub async fn get_hop_stats(state: tauri::State<'_, Arc<Mutex<HopProbeState>>>) -> Result<Vec<HopStat>, String> {
    let state_arc = state.inner().clone();
    let s = state_arc.lock().await;
    let mut results: Vec<HopStat> = s.hops.iter().map(|entry| entry.value().clone()).collect();
    results.sort_by(|a, b| a.hop.cmp(&b.hop));
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hop_stat_updates_success_metrics() {
        let mut stat = HopStat::new(1, "1.1.1.1".to_string());

        stat.update(true, 100.0);
        stat.update(true, 200.0);

        assert_eq!(stat.sent, 2);
        assert_eq!(stat.recv, 2);
        assert_eq!(stat.best_ms, 100.0);
        assert_eq!(stat.worst_ms, 200.0);
        assert_eq!(stat.avg_ms, 150.0);
        assert_eq!(stat.last_ms, 200.0);
        assert_eq!(stat.loss_pct, 0.0);
    }

    #[test]
    fn hop_stat_updates_loss_metrics_with_failures() {
        let mut stat = HopStat::new(2, "2.2.2.2".to_string());

        stat.update(false, 0.0);
        stat.update(true, 80.0);
        stat.update(false, 0.0);

        assert_eq!(stat.sent, 3);
        assert_eq!(stat.recv, 1);
        assert_eq!(stat.best_ms, 80.0);
        assert_eq!(stat.worst_ms, 80.0);
        assert_eq!(stat.avg_ms, 80.0);
        assert!((stat.loss_pct - 66.66666666666666).abs() < 1e-9);
    }

    #[test]
    fn session_activity_respects_epoch_and_cancel_flag() {
        let epoch = AtomicU64::new(7);
        let (tx, rx) = watch::channel(false);

        assert!(is_session_active(&epoch, 7, &rx));

        epoch.store(8, Ordering::SeqCst);
        assert!(!is_session_active(&epoch, 7, &rx));

        epoch.store(7, Ordering::SeqCst);
        let _ = tx.send(true);
        assert!(!is_session_active(&epoch, 7, &rx));
    }
}
