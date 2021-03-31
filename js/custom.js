//import { get_calories_by_day } from "./get_calories_by_day";


var getUserEmail = function () {
    var rtn = "";
    var cname = "email";
    var name = cname + "=";
    var ca = document.cookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            rtn = c.substring(name.length, c.length);
        }
    }
    return rtn;
}

var setUserEmail = function (val) {
    var d = new Date();
    d.setTime(d.getTime() + (365 * 24 * 60 * 60 * 1000));
    var expires = "expires=" + d.toUTCString();
    document.cookie = "email" + "=" + val + ";" + expires + ";path=/";
}



var getUserPw = function () {
    var rtn = "";
    var cname = "pw";
    var name = cname + "=";
    var ca = document.cookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            rtn = c.substring(name.length, c.length);
        }
    }
    return rtn;
}

var setUserPw = function (val) {
    var d = new Date();
    d.setTime(d.getTime() + (365 * 24 * 60 * 60 * 1000));
    var expires = "expires=" + d.toUTCString();
    document.cookie = "pw" + "=" + val + ";" + expires + ";path=/";
}


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

//login
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

//change pw

var change_pw_screen;
var setChangePwScreen = function (fn) {
    change_pw_screen = fn;
}

var changePwScreen = function () {
    change_pw_screen();
}



//change pw

var change_pw;
var setChangePw = function (fn) {
    change_pw = fn;
}

var changePassword = function () {
    change_pw();
}





//register

var register_screen;
var setRegisterScreen = function (fn) {
    register_screen = fn;
}

var registerScreen = function () {
    register_screen();
}



//register

var register;
var setRegister = function (fn) {
    register = fn;
}

var registerUser = function () {
    register();
}



//register

var food_screen;
var setFoodScreen = function (fn) {
    food_screen = fn;
}

var foodScreen = function () {
    food_screen();
}