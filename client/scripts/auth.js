import { concatPath } from "./paths.js";

let mainText = document.getElementById("mainText");
let usernameForm = document.getElementById("usernameInput");
let passwordForm = document.getElementById("passwordInput");

let authBtn = document.getElementById("authBtn");
let registerBtn = document.getElementById("registerBtn");

authBtn.addEventListener("click", () => {
    const username = usernameForm.value.trim();
    const password_hash = passwordForm.value.trim();

    if (!username || !password_hash) {
        mainText.textContent = "Введіть обов'язкові дані!";
        return;
    }

    fetch(concatPath("/users/login"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username,
            password_hash,
            role_name: "user",
        }),
    })
        .then(async (response) => {
            if (!response.ok) {
                throw new Error(
                    `Сервер відповів з помилкою: ${response.status}`
                );
            }

            const isValid = await response.json();

            if (isValid === true) {
                window.location.href =
                    "chat.html?user=" + encodeURIComponent(username);
            } else {
                mainText.textContent = "Невірний логін або пароль!";
            }
        })
        .catch((error) => {
            switch_errors(error, "LOGIN");
        });
});

registerBtn.addEventListener("click", () => {
    let username = usernameForm.value.trim();
    let password_hash = passwordForm.value.trim();

    if (!username || !password_hash) {
        mainText.textContent = "Введіть обов'язкові дані!";
        return;
    }

    fetch(concatPath("/users/register"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username,
            password_hash,
            role_name: "user",
        }),
    })
        .then(async (res) => {
            if (!res.ok) {
                const errorText = await res.text();
                throw { status: res.status, message: errorText };
            }
            return res.json();
        })
        .then((data) => {
            mainText.textContent = `Користувач ${data.username} був успішно зареєстрований!`;
            usernameForm.value = "";
            passwordForm.value = "";
        })
        .catch((error) => switch_errors(error, "REG"));
});

function switch_errors(error, errorType) {
    switch (error.status) {
        case 400:
            mainText.textContent = "Дані заповнені некоректно!";
            break;
        case 409:
            mainText.textContent = "Користувач з таким іменем вже існує!";
            break;
        case 500:
            mainText.textContent = "Сервер помер. Спробуй пізніше.";
            break;
        default:
            mainText.textContent = `Невідома помилка: ${
                error.message || error
            }`;
    }
    console.error(`Oops... you have ${errorType} error: ${error.message}`);
}
