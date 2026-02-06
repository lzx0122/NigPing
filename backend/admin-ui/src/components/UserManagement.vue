<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useUsers } from "../composables/useUsers";
import DeleteConfirm from "./DeleteConfirm.vue";

const {
  users,
  loading,
  error,
  fetchUsers,
  createUser,
  deleteUser,
  resetPassword,
} = useUsers();

// Add user form
const showAddForm = ref(false);
const newUsername = ref("");
const newPassword = ref("");
const autoGenerate = ref(true);

// Generated password display
const showPasswordModal = ref(false);
const generatedPassword = ref("");
const generatedUsername = ref("");

// Delete confirmation
const showDeleteConfirm = ref(false);
const userToDelete = ref<{ id: string; username: string } | null>(null);

// Reset password
const showResetModal = ref(false);
const userToReset = ref<{ id: string; username: string } | null>(null);

onMounted(() => {
  fetchUsers();
});

const handleAddUser = async () => {
  if (!newUsername.value.trim()) {
    alert("Please enter username");
    return;
  }

  if (!autoGenerate.value && !newPassword.value.trim()) {
    alert("Please enter password or enable auto-generate");
    return;
  }

  const result = await createUser({
    username: newUsername.value,
    password: autoGenerate.value ? undefined : newPassword.value,
    autoGenerate: autoGenerate.value,
  });

  if (result) {
    // Show password if auto-generated
    if (result.password) {
      generatedPassword.value = result.password;
      generatedUsername.value = result.user.username;
      showPasswordModal.value = true;
    }

    // Reset form
    newUsername.value = "";
    newPassword.value = "";
    autoGenerate.value = true;
    showAddForm.value = false;
  }
};

const confirmDelete = (id: string, username: string) => {
  userToDelete.value = { id, username };
  showDeleteConfirm.value = true;
};

const handleDelete = async () => {
  if (userToDelete.value) {
    await deleteUser(userToDelete.value.id);
    showDeleteConfirm.value = false;
    userToDelete.value = null;
  }
};

const openResetPassword = (id: string, username: string) => {
  userToReset.value = { id, username };
  showResetModal.value = true;
};

const handleResetPassword = async () => {
  if (userToReset.value) {
    const result = await resetPassword(userToReset.value.id, true);
    if (result && result.password) {
      generatedPassword.value = result.password;
      generatedUsername.value = userToReset.value.username;
      showResetModal.value = false;
      showPasswordModal.value = true;
      userToReset.value = null;
    }
  }
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString("en-US");
};

const copyPassword = () => {
  navigator.clipboard.writeText(generatedPassword.value);
  alert("Password copied to clipboard");
};
</script>

<template>
  <div class="user-management">
    <div class="header">
      <h2>User Management</h2>
      <button class="btn btn-primary" @click="showAddForm = !showAddForm">
        {{ showAddForm ? "Cancel" : "Add User" }}
      </button>
    </div>

    <!-- Add User Form -->
    <Transition name="slide">
      <div v-if="showAddForm" class="add-form">
        <h3>Add User</h3>
        <div class="form-group">
          <label>Username</label>
          <input
            v-model="newUsername"
            type="text"
            placeholder="Enter username"
            class="input"
          />
        </div>

        <div class="form-group">
          <label class="checkbox-label">
            <input v-model="autoGenerate" type="checkbox" />
            <span>Auto-generate password</span>
          </label>
        </div>

        <div v-if="!autoGenerate" class="form-group">
          <label>Password</label>
          <input
            v-model="newPassword"
            type="password"
            placeholder="Enter password"
            class="input"
          />
        </div>

        <div class="form-actions">
          <button class="btn btn-secondary" @click="showAddForm = false">
            Cancel
          </button>
          <button class="btn btn-primary" @click="handleAddUser">Add</button>
        </div>
      </div>
    </Transition>

    <!-- Error Message -->
    <div v-if="error" class="error-message">{{ error }}</div>

    <!-- Loading -->
    <div v-if="loading" class="loading">Loading...</div>

    <!-- Users Table -->
    <div v-else class="table-container">
      <table class="users-table">
        <thead>
          <tr>
            <th>Username</th>
            <th>Created At</th>
            <th>Updated At</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in users" :key="user.id">
            <td class="username">{{ user.username }}</td>
            <td>{{ formatDate(user.created_at) }}</td>
            <td>{{ formatDate(user.updated_at) }}</td>
            <td>
              <span :class="['status', user.is_active ? 'active' : 'inactive']">
                {{ user.is_active ? "Active" : "Inactive" }}
              </span>
            </td>
            <td class="actions">
              <button
                class="btn-icon btn-reset"
                @click="openResetPassword(user.id, user.username)"
                title="Reset Password"
              >
                Reset
              </button>
              <button
                class="btn-icon btn-delete"
                @click="confirmDelete(user.id, user.username)"
                title="Delete"
              >
                Delete
              </button>
            </td>
          </tr>
          <tr v-if="users.length === 0">
            <td colspan="5" class="empty">No users yet</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Password Display Modal -->
    <Transition name="modal">
      <div
        v-if="showPasswordModal"
        class="modal-overlay"
        @click="showPasswordModal = false"
      >
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3>User Password</h3>
          </div>
          <div class="modal-body">
            <p>
              Password generated for user
              <strong>{{ generatedUsername }}</strong
              >:
            </p>
            <div class="password-display">
              <code>{{ generatedPassword }}</code>
              <button class="btn-copy" @click="copyPassword">Copy</button>
            </div>
            <p class="warning">
              Please save this password now. You won't be able to view it again!
            </p>
          </div>
          <div class="modal-actions">
            <button class="btn btn-primary" @click="showPasswordModal = false">
              I've Saved It
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Reset Password Confirmation -->
    <Transition name="modal">
      <div
        v-if="showResetModal"
        class="modal-overlay"
        @click="showResetModal = false"
      >
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3>Reset Password</h3>
          </div>
          <div class="modal-body">
            <p>
              Are you sure you want to reset the password for user
              <strong>{{ userToReset?.username }}</strong
              >?
            </p>
            <p class="warning">A new password will be auto-generated.</p>
          </div>
          <div class="modal-actions">
            <button class="btn btn-secondary" @click="showResetModal = false">
              Cancel
            </button>
            <button class="btn btn-primary" @click="handleResetPassword">
              Confirm Reset
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Delete Confirmation -->
    <DeleteConfirm
      :show="showDeleteConfirm"
      :server-ip="userToDelete?.username || ''"
      @confirm="handleDelete"
      @cancel="showDeleteConfirm = false"
    />
  </div>
</template>

<style scoped>
.user-management {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.header h2 {
  margin: 0;
  font-size: 1.875rem;
  font-weight: 700;
  color: #09090b;
  letter-spacing: -0.025em;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.625rem 1rem;
  border: none;
  border-radius: 0.375rem;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.btn-primary {
  background: #18181b;
  color: #fafafa;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

.btn-primary:hover {
  background: #09090b;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.btn-secondary {
  background: #ffffff;
  color: #18181b;
  border: 1px solid #e4e4e7;
}

.btn-secondary:hover {
  background: #f4f4f5;
  border-color: #d4d4d8;
}

/* Add Form */
.add-form {
  background: #ffffff;
  border: 1px solid #e4e4e7;
  border-radius: 0.5rem;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.add-form h3 {
  margin: 0 0 1.25rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: #09090b;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-group:last-of-type {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: #18181b;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-weight: 400;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
  border-radius: 0.25rem;
  border: 1px solid #e4e4e7;
}

.input {
  width: 100%;
  padding: 0.5rem 0.75rem;
  border: 1px solid #e4e4e7;
  border-radius: 0.375rem;
  font-size: 0.875rem;
  color: #09090b;
  background: #ffffff;
  transition: all 0.15s ease;
}

.input:focus {
  outline: none;
  border-color: #18181b;
  box-shadow: 0 0 0 3px rgba(24, 24, 27, 0.1);
}

.input::placeholder {
  color: #a1a1aa;
}

.form-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid #f4f4f5;
}

/* Table */
.table-container {
  background: #ffffff;
  border: 1px solid #e4e4e7;
  border-radius: 0.5rem;
  overflow: hidden;
  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.users-table {
  width: 100%;
  border-collapse: collapse;
}

.users-table thead {
  background: #fafafa;
  border-bottom: 1px solid #e4e4e7;
}

.users-table th {
  padding: 0.75rem 1rem;
  text-align: left;
  font-size: 0.75rem;
  font-weight: 600;
  color: #71717a;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.users-table td {
  padding: 1rem;
  font-size: 0.875rem;
  color: #18181b;
  border-bottom: 1px solid #f4f4f5;
}

.users-table tbody tr:last-child td {
  border-bottom: none;
}

.users-table tbody tr {
  transition: background-color 0.15s ease;
}

.users-table tbody tr:hover {
  background: #fafafa;
}

.username {
  font-weight: 600;
  color: #09090b;
}

.status {
  display: inline-flex;
  align-items: center;
  padding: 0.25rem 0.625rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  font-weight: 500;
  line-height: 1;
}

.status.active {
  background: #dcfce7;
  color: #166534;
}

.status.inactive {
  background: #fee2e2;
  color: #991b1b;
}

.actions {
  display: flex;
  gap: 0.5rem;
}

.btn-icon {
  padding: 0.375rem 0.75rem;
  border: 1px solid #e4e4e7;
  background: #ffffff;
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  border-radius: 0.375rem;
  transition: all 0.15s ease;
  color: #18181b;
}

.btn-icon:hover {
  background: #f4f4f5;
  border-color: #d4d4d8;
}

.btn-delete {
  color: #dc2626;
  border-color: #fecaca;
}

.btn-delete:hover {
  background: #fee2e2;
  border-color: #fca5a5;
}

.btn-reset {
  color: #2563eb;
  border-color: #bfdbfe;
}

.btn-reset:hover {
  background: #dbeafe;
  border-color: #93c5fd;
}

.empty {
  text-align: center;
  color: #a1a1aa;
  padding: 3rem !important;
  font-size: 0.875rem;
}

.error-message {
  background: #fee2e2;
  color: #991b1b;
  padding: 0.75rem 1rem;
  border-radius: 0.375rem;
  margin-bottom: 1rem;
  font-size: 0.875rem;
  border: 1px solid #fecaca;
}

.loading {
  text-align: center;
  padding: 3rem;
  color: #a1a1aa;
  font-size: 0.875rem;
}

/* Modal */
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
  backdrop-filter: blur(8px);
}

.modal-content {
  background: #ffffff;
  border-radius: 0.5rem;
  max-width: 500px;
  width: 90%;
  box-shadow:
    0 20px 25px -5px rgba(0, 0, 0, 0.1),
    0 10px 10px -5px rgba(0, 0, 0, 0.04);
  overflow: hidden;
}

.modal-header {
  padding: 1.5rem;
  border-bottom: 1px solid #f4f4f5;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: #09090b;
}

.modal-body {
  padding: 1.5rem;
}

.modal-body p {
  margin: 0 0 1rem 0;
  color: #52525b;
  line-height: 1.6;
  font-size: 0.875rem;
}

.modal-body p:last-child {
  margin-bottom: 0;
}

.modal-body strong {
  color: #09090b;
  font-weight: 600;
}

.password-display {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  background: #fafafa;
  padding: 1rem;
  border-radius: 0.375rem;
  margin: 1rem 0;
  border: 1px solid #e4e4e7;
}

.password-display code {
  flex: 1;
  font-family:
    ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono",
    monospace;
  font-size: 1rem;
  font-weight: 600;
  color: #09090b;
  letter-spacing: 0.025em;
}

.btn-copy {
  padding: 0.5rem 0.875rem;
  background: #18181b;
  color: #fafafa;
  border: none;
  border-radius: 0.375rem;
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  white-space: nowrap;
}

.btn-copy:hover {
  background: #09090b;
}

.warning {
  font-size: 0.8125rem;
  color: #dc2626;
  font-weight: 500;
}

.modal-actions {
  padding: 1.5rem;
  background: #fafafa;
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
  border-top: 1px solid #f4f4f5;
}

/* Transitions */
.slide-enter-active,
.slide-leave-active {
  transition: all 0.2s ease;
}

.slide-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

.slide-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
  transition:
    transform 0.2s ease,
    opacity 0.2s ease;
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

@media (max-width: 768px) {
  .user-management {
    padding: 1rem;
  }

  .header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .btn {
    width: 100%;
  }

  .users-table {
    font-size: 0.8125rem;
  }

  .users-table th,
  .users-table td {
    padding: 0.75rem 0.5rem;
  }
}
</style>
