//import { get_calories_by_day } from "./get_calories_by_day";
var get_calories_by_day;
var setCaloriesByDay = function (fn) {
    get_calories_by_day = fn;
}
var getCaloriesByDay = function () {
    get_calories_by_day("ken");
}

var get_food_by_day;
var setFoodByDay = function (fn) {
    get_food_by_day = fn;
}
var getFoodByDay = function () {
    get_food_by_day();
}

var login;
var setLogin = function (fn) {
    login = fn;
}

var loginScreen = function () {
    login();
}

var user_login;
var setLoginUser = function (fn) {
    user_login = fn;
}
var loginUser = function () {
    console.log("in login");
    var email = document.getElementById("email").value;
    console.log(email);
    var pw = document.getElementById("password").value;
    console.log(pw);
    user_login();
}