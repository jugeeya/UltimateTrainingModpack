var slider = document.getElementById('slider');
const STEP = 5

function checkGamepad(index, gamepad) {
    // Gets the current value of the sliders
    var current_value = slider.noUiSlider.get();

    // Displays it on the HTML page
    document.getElementById("input").innerHTML = current_value;


    // Checks to see if the L-button is pressed
    if(gamepad.buttons[4].pressed){
        // If the right-side of the slider is focused on, subtract STEP from the current value
        if(document.activeElement.classList.contains("noUi-handle-upper")){
            slider.noUiSlider.set(
                [
                    null, 
                    parseInt(current_value[1]) - STEP
                ]);
        }
        // If the left-side of the slider is focused on, subtract STEP from the current value
        else if(document.activeElement.classList.contains("noUi-handle-lower")){
            slider.noUiSlider.set(
                [
                    parseInt(current_value[0]) - STEP, 
                    null
                ]);
        }
    }
    // Checks to see if the R-button is pressed
    else if(gamepad.buttons[5].pressed){
        // If the right-side of the slider is focused on, add STEP to the current value
        if(document.activeElement.classList.contains("noUi-handle-upper")){
            slider.noUiSlider.set(
                [
                    null, 
                    parseInt(current_value[1]) + STEP
                ]);
        }
        // If the left-side of the slider is focused on, add STEP to the current value
        else if(document.activeElement.classList.contains("noUi-handle-lower")){
            slider.noUiSlider.set(
                [
                    parseInt(current_value[0]) + STEP,
                    null
                ]);
        }
    }
};

window.onload = function(){
    // Creates the slider
    noUiSlider.create(slider, {
        start: [20, 80],
        connect: true,
        range: {
            'min': 0,
            'max': 100
        }
    });

    // Listens to see if the gamepad is connected
    window.addEventListener("gamepadconnected", function (e) {
        document.getElementById("input").innerHTML = "Gamepad Connected!";
    });

    // Sets an interval and runs a function every 100 seconds to check the gamepads
    setInterval(function () {
        var gpl = navigator.getGamepads();
        if (gpl.length > 0) {
            for (var i = 0; i < gpl.length; i++) {
                checkGamepad(i, gpl[i]);
            }
        }
    }, 100);
}