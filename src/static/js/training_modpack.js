// Polyfill for NodeList.forEach.
// Allows forEach to be called directly on a node list (return type of doucment.querySelectorAll)
if (window.NodeList && !NodeList.prototype.forEach) {
    NodeList.prototype.forEach = Array.prototype.forEach;
}

// Polyfill for Object.entries
// Iterates on an object and returns an array containing arrays of key/value pairs ([key, value])
// for each pair in the object
if (!Object.entries) {
    Object.entries = function (obj) {
        var ownProps = Object.keys(obj),
            i = ownProps.length,
            resArray = new Array(i); // preallocate the Array
        while (i--) resArray[i] = [ownProps[i], obj[ownProps[i]]];

        return resArray;
    };
}

const isNx = typeof window.nx !== 'undefined';
var DEFAULTS_PREFIX = '__';

// Set input handlers
if (isNx) {
    window.nx.footer.setAssign('A', '', function() {document.activeElement.click()}, { se: '' });
    window.nx.footer.setAssign('B', '', closeOrExit, { se: '' });
    window.nx.footer.setAssign('X', '', resetCurrentMenu, { se: '' });
    window.nx.footer.setAssign('L', '', resetAllMenus, { se: '' });
    window.nx.footer.setAssign('R', '', saveDefaults, { se: '' });
    window.nx.footer.setAssign('ZR', '', cycleNextTab, { se: '' });
    window.nx.footer.setAssign('ZL', '', cyclePrevTab, { se: '' });
    window.nx.addEventListener("message", function(msg) { setSettingsFromJSON(msg.data)});
    window.nx.sendMessage("loaded");
} else {
    document.addEventListener('keypress', (event) => {
        switch (event.key) {
            case 'a':
                console.log('a');
                document.activeElement.click();
                break;
            case 'b':
                console.log('b');
                closeOrExit();
                break;
            case 'x':
                console.log('x');
                resetCurrentMenu();
                break;
            case 'l':
                console.log('l');
                resetAllMenus();
                break;
            case 'r':
                console.log('r');
                saveDefaults();
                break;
            case 'p':
                console.log('p');
                cycleNextTab();
                break;
            case 'o':
                console.log('o');
                cyclePrevTab();
                break;
        }
    });
}

const onLoad = () => {
    // Activate the first tab
    openTab(document.querySelector('button.tab-button'));
    initializeAllSliders();
    if (!isNx) {
        settings = {};
        setSettingsFromJSON("{\"menu\":{\"aerial_delay\":0,\"air_dodge_dir\":0,\"attack_angle\":0,\"buff_state\":0,\"character_item\":0,\"clatter_strength\":0,\"crouch\":0,\"di_state\":0,\"falling_aerials\":0,\"fast_fall_delay\":0,\"fast_fall\":0,\"follow_up\":0,\"frame_advantage\":0,\"full_hop\":0,\"hitbox_vis\":1,\"input_delay\":1,\"ledge_delay\":0,\"ledge_state\":31,\"mash_state\":0,\"mash_triggers\":131,\"miss_tech_state\":15,\"oos_offset\":0,\"pummel_delay\":0,\"quick_menu\":0,\"reaction_time\":0,\"save_damage\":4,\"save_damage_limits\":[63,106],\"save_state_autoload\":1,\"save_state_enable\":1,\"save_state_mirroring\":1,\"sdi_state\":0,\"sdi_strength\":0,\"shield_state\":0,\"shield_tilt\":0,\"stage_hazards\":0,\"tech_state\":15,\"throw_delay\":0,\"throw_state\":1},\"defaults_menu\":{\"aerial_delay\":0,\"air_dodge_dir\":0,\"attack_angle\":0,\"buff_state\":0,\"character_item\":0,\"clatter_strength\":0,\"crouch\":0,\"di_state\":0,\"falling_aerials\":0,\"fast_fall_delay\":0,\"fast_fall\":0,\"follow_up\":0,\"frame_advantage\":0,\"full_hop\":0,\"hitbox_vis\":1,\"input_delay\":1,\"ledge_delay\":0,\"ledge_state\":31,\"mash_state\":0,\"mash_triggers\":131,\"miss_tech_state\":15,\"oos_offset\":0,\"pummel_delay\":0,\"quick_menu\":0,\"reaction_time\":0,\"save_damage\":4,\"save_damage_limits\":[41,118],\"save_state_autoload\":1,\"save_state_enable\":1,\"save_state_mirroring\":1,\"sdi_state\":0,\"sdi_strength\":0,\"shield_state\":0,\"shield_tilt\":0,\"stage_hazards\":0,\"tech_state\":15,\"throw_delay\":0,\"throw_state\":1}}");
    }
};

window.onload = onLoad;

var settings;
var defaultSettings;

var lastFocusedItem = document.querySelector('.menu-item > button');
const currentTabContent = () => {
    const currentActiveTab = document.querySelector('.tab-button.active');

    var currentActiveTabContent = document.querySelector(`#${currentActiveTab.id.replace('button', 'tab')}`);

    return currentActiveTabContent;
};

const openTab = (eventTarget) => {
    playSound('SeWebZoomIn');
    const selectedTab = document.getElementById(eventTarget.id.replace('button', 'tab'));
    const activeTabContent = document.querySelector('.tab-content:not(.hide)');
    const activeTab = document.querySelector('.tab-button.active');

    // Hide content of current active tab
    if (activeTabContent) {
        activeTabContent.classList.add('hide');
    }

    closeAllActiveModals();

    // Remove "active" class from current active tab
    if (activeTab) {
        activeTab.classList.remove('active');
    }

    // Show the new current tab, and add an "active" class to the button that opened the tab
    eventTarget.classList.add('active');
    selectedTab.classList.remove('hide');
    selectedTab.querySelector('button').focus();
};

const openMenuItem = (eventTarget) => {
    playSound('SeWebMenuListOpen');

    var { target } = eventTarget.dataset;
    const modal = document.querySelector(`.modal[data-id=${target}]`);

    currentTabContent().classList.toggle('hide');

    modal.classList.toggle('hide');
    elem = modal.querySelector('button');
    if (!elem) {
        elem = modal.querySelector('.noUi-handle-lower')
    }
    elem.focus();

    lastFocusedItem = eventTarget;
};

const closeAllActiveModals = () => {
    document.querySelectorAll('.modal:not(.hide)').forEach((modal) => {
        modal.classList.add('hide');
    });
    lastFocusedItem.focus();
};

const toggleOption = (element) => {
    playSound('SeSelectCheck');

    if (element.parentElement.classList.contains('single-option')) {
        selectSingleOption(element);
        return;
    }

    const img = element.querySelector('img');
    const previouslySelected = !img.classList.contains('hidden');
    const menuId = element.parentElement.dataset.id;
    const toggleValue = parseInt(img.dataset.val);

    settings[menuId] = previouslySelected ? settings[menuId] - toggleValue : settings[menuId] + toggleValue;

    element.querySelector('img').classList.toggle('hidden');
};

// Add this later
// function toggleSingleOption(element) {
//     selectSingleOption(element);
// }

const closestClass = (element, class_) => {
    if (!element) {
        return null;
    }

    if (element.classList.contains(class_)) {
        return element;
    }

    // Didn't find it, go up a level
    return closestClass(element.parentElement, class_);
};

function playSound(label) {
    //** Valid labels **//
    // SeToggleBtnFocus, SeToggleBtnOn, SeToggleBtnOff, SeCheckboxFocus, SeCheckboxOn
    // SeCheckboxOff, SeRadioBtnFocus, SeRadioBtnOn, SeSelectCheck, SeSelectUncheck, SeBtnDecide
    // SeTouchUnfocus, SeBtnFocus, SeKeyError, SeDialogOpen, SeWebZoomOut, SeWebZoomIn, SeWebNaviFocus
    // SeWebPointerFocus, SeFooterFocus, SeFooterDecideBack, SeFooterDecideFinish, SeWebChangeCursorPointer
    // SeWebTouchFocus, SeWebLinkDecide, SeWebTextboxStartEdit, SeWebButtonDecide, SeWebRadioBtnOn
    // SeWebCheckboxUncheck, SeWebCheckboxCheck, SeWebMenuListOpen

    if (isNx) {
        window.nx.playSystemSe(label);
    } else {
        console.log('Sound Effect: ' + label);
    }
}

const exit = () => {
    playSound('SeFooterDecideBack');
    const messageObject = {
        menu: settings,
        defaults_menu: defaultSettings
    }

    if (isNx) {
        window.nx.sendMessage(
            JSON.stringify(messageObject)
        );
    } else {
        console.log(JSON.stringify(messageObject));
    }
};

function closeOrExit() {
    // Close any open menus
    if (document.querySelector('.modal:not(.hide)')) {
        console.log('Closing Items');
        closeAllActiveModals();
        currentTabContent().classList.remove('hide');
        lastFocusedItem.focus();
        return;
    }

    console.log('Exiting');
    exit();
}

function setSettingsFromJSON(msg) {
    // Receive a menu message and set settings
    var msg_json = JSON.parse(msg);
    settings = msg_json["menu"];
    defaultSettings = msg_json["defaults_menu"];
    populateMenuFromSettings();
}

function selectSingleOption(eventTarget) {
    // Deselect all options in the submenu
    const parent = eventTarget.parentElement;

    parent.querySelectorAll('.menu-icon:not(.hidden)').forEach((sibling) => {
        sibling.classList.add('hidden');
        settings[parent.dataset.id] = settings[parent.dataset.id] - parseInt(sibling.dataset.val);
    });

    eventTarget.querySelector('.menu-icon').classList.remove('hidden');

    const value = parseInt(eventTarget.querySelector('.menu-icon').dataset.val);

    settings[parent.dataset.id] = settings[parent.dataset.id] + value;
}

const isValueInBitmask = (value, mask) => (mask & value) != 0;

const setOptionsForMenu = (menuId) => {
    const modal = document.querySelector(`.modal[data-id="${menuId}"]`);

    if (modal.querySelector('.modal-button')) {
        // Toggle menu
        modal.querySelectorAll('.menu-icon').forEach(function (toggle) {
            if (isValueInBitmask(toggle.dataset.val, settings[menuId])) {
                toggle.classList.remove('hidden');
            } else {
                toggle.classList.add('hidden');
            }
        });
    
        if (modal.classList.contains('single-option')) {
            // If no option is selected default to the first option
            if (modal.querySelectorAll('.menu-icon:not(.hidden)').length === 0) {
                selectSingleOption(modal.querySelector('button'));
            }
        }
    } else {
        // Slider menu
        slider = modal.querySelector('.modal-slider');
        setSliderVals(slider, settings[menuId]);
    }
};

function populateMenuFromSettings() {
    document.querySelectorAll('.menu-item').forEach((item) => setOptionsForMenu(item.id));
}

function getSettingsValFromMenuID(id) {
    const modal = document.querySelector(`.modal[data-id='${id}']`);

    if (modal.querySelector('.modal-button')) {
        // Toggle menu
        // Return value is a bitmask
        var value = 0;
        const options = modal.querySelectorAll('img:not(.hidden)');
    
        options.forEach(function (toggle) {
            value += parseInt(toggle.dataset.val);
        });
        return value;
    } else {
        // Slider menu
        // Return value is a [lower,upper] array
        slider = modal.querySelector('.modal-slider');
        return getSliderVals(slider);
    }
}

function resetCurrentMenu() {
    playSound('SeWebTextboxStartEdit');
    const menu = document.querySelector('.modal:not(.hide)');

    const menuId = menu.dataset.id;
    const defaultSubmenuSetting = defaultSettings[menuId];

    settings[menuId] = defaultSubmenuSetting;

    populateMenuFromSettings();
}

function resetAllMenus() {
    // Resets all submenus to the default values
    if (confirm('Are you sure that you want to reset all menu settings to the default?')) {
        document.querySelectorAll('.menu-item').forEach(function (item) {
            const defaultMenuId = item.id;
            const defaultSubmenuSetting = defaultSettings[defaultMenuId];

            settings[item.id] = defaultSubmenuSetting;

            populateMenuFromSettings();
        });
    }
}

function setHelpText(text) {
    document.getElementById('help-text').innerText = text;
}

function saveDefaults() {
    if (confirm('Are you sure that you want to change the default menu settings to the current selections?')) {
        document.querySelectorAll('.menu-item').forEach((item) => {
            const menu = item.id;
            defaultSettings[menu] = getSettingsValFromMenuID(item.id);
        });
    }
}

function cycleNextTab() {
    // Cycle to the next tab
    const activeTab = document.querySelector('.tab-button.active');
    var nextTab = activeTab.nextElementSibling;
    if (!nextTab) {
        // On the last tab - set the next tab as the first tab in the list
        nextTab = document.querySelector('.tab-button');
    }
    openTab(nextTab);
}

function cyclePrevTab() {
    // Cycle to the previous tab
    const activeTab = document.querySelector('.tab-button.active');
    var previousTab = activeTab.previousElementSibling;
    if (!previousTab) {
        // On the first tab - set the next tab as the last tab in the list
        tabs = document.querySelectorAll('.tab-button');
        previousTab = tabs[tabs.length - 1];
    }
    openTab(previousTab);
}

function getSliderVals(slider) {
    var arr = slider.noUiSlider.get();
    return [parseFloat(arr[0]), parseFloat(arr[1])]
}

function setSliderVals(slider, vals) {
    slider.noUiSlider.set(vals);
}

function setSettingsFromSlider(slider) {
    menuId = closestClass(slider, "modal").dataset.id;
    settings[menuId] = getSliderVals(slider)
}

function initializeSlider(slider) {
    noUiSlider.create(
        slider,
        {
            start: [
                parseFloat(slider.dataset.selectedMin),
                parseFloat(slider.dataset.selectedMax),
            ],
            connect: true,
            range: {
                'min': parseFloat(slider.dataset.absMin),
                'max': parseFloat(slider.dataset.absMax),
            },
            step: 1,
            tooltips: [
                {to: function (value) {return value.toFixed(0) + '%';}},
                {to: function (value) {return value.toFixed(0) + '%';}},
            ],
            pips: {
                mode: 'range',
                density: 10,
            }
        }
    );
    slider.noUiSlider.on('set', function() {setSettingsFromSlider(slider)});
}

function initializeAllSliders() {
    document.querySelectorAll(".modal-slider").forEach((item) => {
        initializeSlider(item);
    });
}