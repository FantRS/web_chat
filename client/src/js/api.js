class APIClient {
    constructor(baseURL = "http://localhost:3000/api") {
        this.baseURL = baseURL;
    }

    /**
     * Базовий метод для виконання запитів
     * @param {string} endpoint - шлях до ендпоінту
     * @param {string} method - HTTP метод
     * @param {object} body - дані для відправки
     * @returns {Promise}
     */
    async request(endpoint, method = "GET", body = null) {
        try {
            const options = {
                method,
                headers: {
                    "Content-Type": "application/json",
                },
            };

            if (body) {
                options.body = JSON.stringify(body);
            }

            const token = localStorage.getItem("authToken");
            if (token) {
                options.headers["Authorization"] = `Bearer ${token}`;
            }

            const response = await fetch(`${this.baseURL}${endpoint}`, options);
            const data = await response.json();

            if (!response.ok) {
                throw new Error(data.message || "API Error");
            }

            return data;
        } catch (error) {
            console.error("API Error:", error);
            throw error;
        }
    }

    /**
     * Реєстрація нового користувача
     * @param {object} userData - дані користувача
     * @returns {Promise}
     */
    async register(userData) {
        return this.request("/users/register", "POST", userData);
    }

    /**
     * Логін користувача
     * @param {string} email - email користувача
     * @param {string} password - пароль користувача
     * @returns {Promise}
     */
    async login(email, password) {
        return this.request("/users/login", "POST", { email, password });
    }

    /**
     * Логаут користувача
     * @returns {Promise}
     */
    async logout() {
        localStorage.removeItem("authToken");
        return Promise.resolve({ success: true });
    }

    /**
     * Отримати дані поточного користувача
     * @returns {Promise}
     */
    async getCurrentUser() {
        return this.request("/users/me", "GET");
    }

    /**
     * Оновити профіль користувача
     * @param {object} userData - оновлені дані
     * @returns {Promise}
     */
    async updateProfile(userData) {
        return this.request("/users/profile", "PUT", userData);
    }
}

// Глобальна екземпляр API клієнта
const api = new APIClient();
