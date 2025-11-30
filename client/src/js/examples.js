/**
 * ПРИКЛАДИ ВИКОРИСТАННЯ
 * Як використовувати API та обробку форм
 */

// ============================================
// 1. БАЗОВІ ЗАПИТИ ДО API
// ============================================

// Реєстрація користувача
async function registerUser() {
  try {
    const response = await api.register({
      email: 'newuser@example.com',
      password: 'securePassword123',
      name: 'John Doe',
      age: 25,
      description: 'I love web development!'
    });
    
    console.log('Registration successful:', response);
    // response містить дані користувача та токен
  } catch (error) {
    console.error('Registration failed:', error.message);
  }
}

// Логін користувача
async function loginUser() {
  try {
    const response = await api.login(
      'user@example.com',
      'password123'
    );
    
    console.log('Login successful:', response);
    // Токен автоматично зберігається в localStorage
  } catch (error) {
    console.error('Login failed:', error.message);
  }
}

// Отримати дані поточного користувача
async function getUserProfile() {
  try {
    const response = await api.getCurrentUser();
    console.log('User profile:', response);
  } catch (error) {
    console.error('Failed to get profile:', error.message);
  }
}

// Оновити профіль
async function updateUserProfile() {
  try {
    const response = await api.updateProfile({
      name: 'Jane Doe',
      age: 26,
      description: 'Updated bio'
    });
    
    console.log('Profile updated:', response);
  } catch (error) {
    console.error('Profile update failed:', error.message);
  }
}

// Логаут
async function logoutUser() {
  api.logout();
  window.location.href = './index.html';
}

// ============================================
// 2. РОБОТА З ЛОКАЛЬНИМ СХОВИЩЕМ
// ============================================

// Зберегти токен
function saveToken(token) {
  localStorage.setItem('authToken', token);
}

// Отримати токен
function getToken() {
  return localStorage.getItem('authToken');
}

// Очистити токен
function clearToken() {
  localStorage.removeItem('authToken');
}

// Перевірити чи користувач залогінений
function isUserLoggedIn() {
  return !!localStorage.getItem('authToken');
}

// ============================================
// 3. ВАЛІДАЦІЯ ДАНИХ
// ============================================

// Валідація email
function isValidEmail(email) {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return regex.test(email);
}

// Валідація пароля
function isValidPassword(password) {
  return password.length >= 6;
}

// Валідація ім'я
function isValidName(name) {
  return name.trim().length >= 2;
}

// Валідація віку
function isValidAge(age) {
  const ageNum = parseInt(age);
  return ageNum >= 13 && ageNum <= 120;
}

// ============================================
// 4. УТИЛІТНІ ФУНКЦІЇ
// ============================================

// Показ повідомлення про помилку
function showErrorMessage(message) {
  const errorDiv = document.createElement('div');
  errorDiv.className = 'error-notification';
  errorDiv.textContent = message;
  document.body.appendChild(errorDiv);
  
  setTimeout(() => {
    errorDiv.remove();
  }, 5000);
}

// Показ повідомлення про успіх
function showSuccessMessage(message) {
  const successDiv = document.createElement('div');
  successDiv.className = 'success-notification';
  successDiv.textContent = message;
  document.body.appendChild(successDiv);
  
  setTimeout(() => {
    successDiv.remove();
  }, 5000);
}

// Отримати дані форми в об'єкт
function getFormData(formId) {
  const form = document.getElementById(formId);
  const formData = new FormData(form);
  return Object.fromEntries(formData);
}

// Очистити форму
function clearForm(formId) {
  document.getElementById(formId).reset();
}

// ============================================
// 5. ОБРОБКА ПОМИЛОК МЕРЕЖІ
// ============================================

async function apiCallWithRetry(apiFunction, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await apiFunction();
    } catch (error) {
      if (i === maxRetries - 1) {
        throw error;
      }
      // Очікати перед повторною спробою (експоненціальна затримка)
      await new Promise(resolve => 
        setTimeout(resolve, Math.pow(2, i) * 1000)
      );
    }
  }
}

// ============================================
// 6. ПРИКЛАДИ ВИКОРИСТАННЯ ФОРМ
// ============================================

// Приклад: Обробка кастомної форми
document.addEventListener('DOMContentLoaded', () => {
  // Приклад логіну
  const loginForm = document.getElementById('loginForm');
  if (loginForm) {
    loginForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      
      const email = document.getElementById('email').value;
      const password = document.getElementById('password').value;
      
      // Валідація
      if (!isValidEmail(email)) {
        showErrorMessage('Неправильний формат email');
        return;
      }
      
      if (!isValidPassword(password)) {
        showErrorMessage('Пароль занадто короткий');
        return;
      }
      
      // Відправка запиту
      try {
        await apiCallWithRetry(() => api.login(email, password));
        showSuccessMessage('Успішний вход!');
        setTimeout(() => {
          window.location.href = '/dashboard';
        }, 1000);
      } catch (error) {
        showErrorMessage(error.message);
      }
    });
  }
  
  // Приклад реєстрації
  const registerForm = document.getElementById('registerForm');
  if (registerForm) {
    registerForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      
      const formData = {
        email: document.getElementById('email').value,
        password: document.getElementById('password').value,
        confirmPassword: document.getElementById('confirmPassword').value,
        name: document.getElementById('name').value,
        age: document.getElementById('age').value,
        description: document.getElementById('description').value
      };
      
      // Валідація
      const errors = [];
      
      if (!isValidEmail(formData.email)) {
        errors.push('Неправильний email');
      }
      if (!isValidPassword(formData.password)) {
        errors.push('Пароль занадто короткий');
      }
      if (formData.password !== formData.confirmPassword) {
        errors.push('Паролі не збігаються');
      }
      if (!isValidName(formData.name)) {
        errors.push("Ім'я занадто коротке");
      }
      if (!isValidAge(formData.age)) {
        errors.push('Невірний вік');
      }
      
      if (errors.length > 0) {
        errors.forEach(error => showErrorMessage(error));
        return;
      }
      
      // Відправка запиту
      try {
        await apiCallWithRetry(() => api.register({
          email: formData.email,
          password: formData.password,
          name: formData.name,
          age: parseInt(formData.age),
          description: formData.description
        }));
        
        showSuccessMessage('Реєстрація успішна!');
        setTimeout(() => {
          window.location.href = './login.html';
        }, 1000);
      } catch (error) {
        showErrorMessage(error.message);
      }
    });
  }
});

// ============================================
// 7. ПРАКТИЧНІ ПРИКЛАДИ
// ============================================

// Перевірити сесію при завантаженні сторінки
async function checkUserSession() {
  if (!isUserLoggedIn()) {
    window.location.href = './index.html';
    return;
  }
  
  try {
    const user = await api.getCurrentUser();
    console.log('Current user:', user);
    return user;
  } catch (error) {
    // Токен невалідний, перенаправляємо на логін
    clearToken();
    window.location.href = './index.html';
  }
}

// Заповнити форму попередніми даними
function populateFormWithUserData(user) {
  if (user.email) document.getElementById('email').value = user.email;
  if (user.name) document.getElementById('name').value = user.name;
  if (user.age) document.getElementById('age').value = user.age;
  if (user.description) document.getElementById('description').value = user.description;
}

// ============================================
// 8. КОНФІГУРАЦІЯ API
// ============================================

// Якщо вам потрібно змінити URL сервера:
// const api = new APIClient('http://your-custom-server.com/api');

// Або встановити інші параметри:
// api.baseURL = 'http://localhost:8080/api';
