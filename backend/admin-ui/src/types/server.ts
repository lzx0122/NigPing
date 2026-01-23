export interface Server {
  ip: string;
  region: string;
  addedAt: string;
}

export interface ServerStats {
  total: number;
  byRegion: Record<string, number>;
}
