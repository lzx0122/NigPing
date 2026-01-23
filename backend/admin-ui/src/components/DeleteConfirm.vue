<script setup lang="ts">
defineProps<{
  serverIp: string;
  show: boolean;
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();
</script>

<template>
  <Transition name="modal">
    <div v-if="show" class="modal-overlay" @click="emit('cancel')">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Confirm Deletion</h3>
        </div>
        <div class="modal-body">
          <p>
            Are you sure you want to delete server
            <strong>{{ serverIp }}</strong
            >?
          </p>
          <p class="warning">This action cannot be undone.</p>
        </div>
        <div class="modal-actions">
          <button class="btn btn-secondary" @click="emit('cancel')">
            Cancel
          </button>
          <button class="btn btn-danger" @click="emit('confirm')">
            Delete
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-content {
  background: #ffffff;
  border-radius: 16px;
  max-width: 440px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.modal-header {
  padding: 1.5rem 1.75rem;
  border-bottom: 1px solid #e7e5e4;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #292524;
}

.modal-body {
  padding: 1.75rem;
}

.modal-body p {
  margin: 0 0 0.75rem 0;
  color: #57534e;
  line-height: 1.6;
}

.modal-body p:last-child {
  margin-bottom: 0;
}

.modal-body strong {
  color: #292524;
  font-weight: 600;
}

.warning {
  font-size: 0.875rem;
  color: #dc2626;
  font-weight: 500;
}

.modal-actions {
  padding: 1.25rem 1.75rem;
  background: #fafaf9;
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 8px;
  font-size: 0.9375rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-secondary {
  background: #ffffff;
  color: #57534e;
  border: 1.5px solid #d6d3d1;
}

.btn-secondary:hover {
  background: #f5f5f4;
  border-color: #a8a29e;
}

.btn-danger {
  background: #dc2626;
  color: #ffffff;
}

.btn-danger:hover {
  background: #b91c1c;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(220, 38, 38, 0.25);
}

/* Modal transitions */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.25s ease;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition:
    transform 0.25s ease,
    opacity 0.25s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.95);
  opacity: 0;
}
</style>
