/**
 * Form Handler Module
 * Обробка всіх форм на сторінках
 */

class FormHandler {
    constructor() {
        this.init();
    }

    /**
     * Ініціалізація обробників форм
     */
    init() {
        // На сторінці логіну
        const loginForm = document.getElementById("loginForm");
        if (loginForm) {
            loginForm.addEventListener("submit", (e) => this.handleLogin(e));
        }

        const registerBtn = document.getElementById("registerBtn");
        if (registerBtn) {
            registerBtn.addEventListener("click", () => this.goToRegister());
        }

        // На сторінці реєстрації
        const registerForm = document.getElementById("registerForm");
        if (registerForm) {
            registerForm.addEventListener("submit", (e) =>
                this.handleRegister(e)
            );
        }

        const backBtn = document.getElementById("backBtn");
        if (backBtn) {
            backBtn.addEventListener("click", () => this.goToLogin());
        }
    }

    /**
     * Обробка форми логіну
     */
    async handleLogin(e) {
        e.preventDefault();

        const email = document.getElementById("email").value.trim();
        const password = document.getElementById("password").value;

        // Валідація
        if (!this.validateEmail(email)) {
            this.showError("emailError", "Введіть коректний email");
            return;
        }

        if (password.length < 6) {
            this.showError(
                "passwordError",
                "Пароль повинен містити щонайменше 6 символів"
            );
            return;
        }

        try {
            this.clearErrors();
            const button = document.querySelector(
                '#loginForm button[type="submit"]'
            );
            button.disabled = true;
            button.classList.add("loading");

            const response = await api.login(email, password);

            // Зберігаємо токен
            if (response.token) {
                localStorage.setItem("authToken", response.token);
                this.showSuccess("Успішний вход!");

                // Перенаправляємо на головну сторінку після 1 секунди
                setTimeout(() => {
                    window.location.href = "/dashboard"; // Змініть URL на ваш
                }, 1000);
            }
        } catch (error) {
            this.showError(
                "errorMessage",
                error.message || "Помилка при вході. Перевірте дані."
            );
        } finally {
            const button = document.querySelector(
                '#loginForm button[type="submit"]'
            );
            button.disabled = false;
            button.classList.remove("loading");
        }
    }

    /**
     * Обробка форми реєстрації
     */
    async handleRegister(e) {
        e.preventDefault();

        const email = document.getElementById("email").value.trim();
        const password = document.getElementById("password").value;
        const confirmPassword =
            document.getElementById("confirmPassword").value;
        const name = document.getElementById("name").value.trim();
        const age = parseInt(document.getElementById("age").value);
        const description = document.getElementById("description").value.trim();

        // Валідація
        if (!this.validateEmail(email)) {
            this.showError("emailError", "Введіть коректний email");
            return;
        }

        if (password.length < 6) {
            this.showError(
                "passwordError",
                "Пароль повинен містити щонайменше 6 символів"
            );
            return;
        }

        if (password !== confirmPassword) {
            this.showError("confirmPasswordError", "Паролі не збігаються");
            return;
        }

        if (name.length < 2) {
            this.showError(
                "nameError",
                "Ім'я повинно містити щонайменше 2 символи"
            );
            return;
        }

        if (age < 13 || age > 120) {
            this.showError("ageError", "Вік повинен бути від 13 до 120 років");
            return;
        }

        try {
            this.clearErrors();
            const button = document.querySelector(
                '#registerForm button[type="submit"]'
            );
            button.disabled = true;
            button.classList.add("loading");

            const userData = {
                email,
                password,
                name,
                age,
                description: description || null,
            };

            const response = await api.register(userData);

            this.showSuccess(
                "Аккаунт успішно створений! Перенаправляємо на логін..."
            );

            // Перенаправляємо на логін після 1.5 секунди
            setTimeout(() => {
                window.location.href = "./login.html";
            }, 1500);
        } catch (error) {
            this.showError(
                "errorMessage",
                error.message || "Помилка при реєстрації. Спробуйте ще раз."
            );
        } finally {
            const button = document.querySelector(
                '#registerForm button[type="submit"]'
            );
            button.disabled = false;
            button.classList.remove("loading");
        }
    }

    /**
     * Перехід на сторінку реєстрації
     */
    goToRegister() {
        window.location.href = "./src/pages/register.html";
    }

    /**
     * Повернення на сторінку логіну
     */
    goToLogin() {
        window.location.href = "./login.html";
    }

    /**
     * Валідація email
     */
    validateEmail(email) {
        const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return re.test(email);
    }

    /**
     * Показати повідомлення про помилку
     */
    showError(elementId, message) {
        const errorElement = document.getElementById(elementId);
        if (errorElement) {
            errorElement.textContent = message;
            errorElement.classList.add("show");
        }
    }

    /**
     * Показати повідомлення про успіх
     */
    showSuccess(message) {
        const successElement = document.getElementById("successMessage");
        if (successElement) {
            successElement.textContent = message;
            successElement.classList.add("show");
        }
    }

    /**
     * Очистити всі помилки
     */
    clearErrors() {
        const errorElements = document.querySelectorAll(".error");
        errorElements.forEach((el) => {
            el.classList.remove("show");
            el.textContent = "";
        });

        const successElement = document.getElementById("successMessage");
        if (successElement) {
            successElement.classList.remove("show");
            successElement.textContent = "";
        }
    }
}

// Ініціалізуємо обробник форм коли сторінка завантажена
document.addEventListener("DOMContentLoaded", () => {
    new FormHandler();
});
