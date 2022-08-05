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
    window.nx.footer.setAssign('B', '', closeOrExit, { se: '' });
    window.nx.footer.setAssign('X', '', resetCurrentMenu, { se: '' });
    window.nx.footer.setAssign('L', '', resetAllMenus, { se: '' });
    window.nx.footer.setAssign('R', '', saveDefaults, { se: '' });
    window.nx.footer.setAssign('ZR', '', cycleNextTab, { se: '' });
    window.nx.footer.setAssign('ZL', '', cyclePrevTab, { se: '' });
} else {
    document.addEventListener('keypress', (event) => {
        switch (event.key) {
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
                resetAllSubmenus();
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

    // Extract URL params and set appropriate settings
    setSettingsFromURL();
    populateMenuFromSettings();
};

window.onload = onLoad;

var settings;

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
    modal.querySelector('button').focus();

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

    const url = buildURLFromSettings();


    if (isNx) {
        window.nx.sendMessage(url);
        window.location.href = url;
    } else {
        console.log(url);
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

function setSettingsFromURL() {
    var { search } = window.location;
    const settingsFromSearch = search
        .replace('?', '')
        .split('&')
        .reduce((accumulator, currentValue) => {
            var [key, value] = currentValue.split('=');
            accumulator[key] = parseInt(value);
            return accumulator;
        }, {});

    settings = settingsFromSearch;
}

function buildURLFromSettings() {
    const url = 'http://localhost/?';

    const urlParams = Object.entries(settings).map((setting) => {
        return `${setting[0]}=${setting[1]}`;
    });

    return url + urlParams.join('&');
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
};

function populateMenuFromSettings() {
    document.querySelectorAll('.menu-item').forEach((item) => setOptionsForMenu(item.id));
}

function getMaskFromMenuID(id) {
    var value = 0;
    const modal = document.querySelector(`.modal[data-id='${id}']`);

    const options = modal.querySelectorAll('img:not(.hidden)');

    options.forEach(function (toggle) {
        value += parseInt(toggle.dataset.val);
    });

    return value;
}

function resetCurrentMenu() {
    playSound('SeWebTextboxStartEdit');
    const menu = document.querySelector('.modal:not(.hide)');

    const menuId = menu.dataset.id;
    const defaultSectionMask = settings[DEFAULTS_PREFIX + menuId];

    settings[menuId] = defaultSectionMask;

    populateMenuFromSettings();
}

function resetAllMenus() {
    // Resets all submenus to the default values
    if (confirm('Are you sure that you want to reset all menu settings to the default?')) {
        document.querySelectorAll('.menu-item').forEach(function (item) {
            const defaultMenuId = DEFAULTS_PREFIX + item.id;
            const defaultMask = settings[defaultMenuId];

            settings[item.id] = defaultMask;

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
            const menu = DEFAULTS_PREFIX + item.id;

            settings[menu] = getMaskFromMenuID(item.id);
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
