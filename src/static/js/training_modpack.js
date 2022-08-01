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

var isNx = typeof window.nx !== 'undefined';
var defaults_prefix = '__';

// Set input handlers
if (isNx) {
    window.nx.footer.setAssign('B', '', close_or_exit, { se: '' });
    window.nx.footer.setAssign('X', '', resetCurrentSubmenu, { se: '' });
    window.nx.footer.setAssign('L', '', resetAllSubmenus, { se: '' });
    window.nx.footer.setAssign('R', '', saveDefaults, { se: '' });
    window.nx.footer.setAssign('ZR', '', cycleNextTab, { se: '' });
    window.nx.footer.setAssign('ZL', '', cyclePrevTab, { se: '' });
} else {
    document.addEventListener('keypress', (event) => {
        switch (event.key) {
            case 'b':
                console.log('b');
                close_or_exit();
                break;
            case 'x':
                console.log('x');
                resetCurrentSubmenu();
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

window.onload = onLoad;

var settings;

var lastFocusedItem = document.querySelector('.menu-item > button');
var currentTabContent = () => {
    var currentActiveTab = document.querySelector('.tab-button.active');

    var currentActiveTabContent = document.querySelector(`#${currentActiveTab.id.replace('button', 'tab')}`);

    return currentActiveTabContent;
};

function onLoad() {
    // Activate the first tab
    openTab(document.querySelector('button.tab-button'));

    // Extract URL params and set appropriate settings
    setSettingsFromURL();
    populateMenuFromSettings();
}

function openTab(eventTarget) {
    var selectedTab = document.getElementById(eventTarget.id.replace('button', 'tab'));
    var activeTabContent = document.querySelector('.tab-content:not(.hide)');
    var activeTab = document.querySelector('.tab-button.active');

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
}

function openMenuItem(eventTarget) {
    playSound('SeWebMenuListOpen');

    var { target } = eventTarget.dataset;
    var modal = document.querySelector(`.modal[data-id=${target}]`);

    currentTabContent().classList.toggle('hide');

    modal.classList.toggle('hide');
    modal.querySelector('button').focus();

    lastFocusedItem = eventTarget;
}

function closeAllActiveModals() {
    document.querySelectorAll('.modal:not(.hide)').forEach((modal) => {
        modal.classList.add('hide');
    });
    lastFocusedItem.focus();
}

function toggleOption(element) {
    playSound('SeSelectCheck');

    if (element.parentElement.classList.contains('single-option')) {
        selectSingleOption(element);
        return;
    }

    var img = element.querySelector('img');
    var previouslySelected = !img.classList.contains('hidden');
    var menuId = element.parentElement.dataset.id;
    var toggleValue = parseInt(img.dataset.val);

    settings[menuId] = previouslySelected ? settings[menuId] - toggleValue : settings[menuId] + toggleValue;

    element.querySelector('img').classList.toggle('hidden');
}

// Add this later
// function toggleSingleOption(element) {
//     selectSingleOption(element);
// }

function closestClass(element, class_) {
    // Returns the closest ancestor (including self) with the given class
    if (!element) {
        // Reached the end of the DOM
        return null;
    }

    if (element.classList.contains(class_)) {
        // Found it
        return element;
    } else {
        // Didn't find it, go up a level
        return closestClass(element.parentElement, class_);
    }
}

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

function exit() {
    playSound('SeFooterDecideBack');

    var url = buildURLFromSettings();

    if (isNx) {
        window.location.href = url;
    } else {
        console.log(url);
    }
}

function close_or_exit() {
    // If any submenus are open, close them
    // Otherwise if all submenus are closed, exit the menu and return to the game

    if (document.querySelector('.modal:not(.hide)')) {
        // Close any open submenus
        console.log('Closing Items');
        closeAllActiveModals();
        currentTabContent().classList.remove('hide');
        lastFocusedItem.focus();
    } else {
        // If all submenus are closed, exit and return through localhost
        console.log('Exiting');
        exit();
    }
}

function setSettingsFromURL() {
    var { search } = window.location;
    var settingsFromSearch = search
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
    var url = 'http://localhost/?';

    var urlParams = Object.entries(settings).map((setting) => {
        return `${setting[0]}=${setting[1]}`;
    });

    return url + urlParams.join('&');
}

function selectSingleOption(eventTarget) {
    // Deselect all options in the submenu
    var parent = eventTarget.parentElement;

    parent.querySelectorAll('.menu-icon:not(.hidden)').forEach((sibling) => {
        sibling.classList.add('hidden');
        settings[parent.dataset.id] = settings[parent.dataset.id] - parseInt(sibling.dataset.val);
    });

    eventTarget.querySelector('.menu-icon').classList.remove('hidden');

    var value = parseInt(eventTarget.querySelector('.menu-icon').dataset.val);

    settings[parent.dataset.id] = settings[parent.dataset.id] + value;
}

function populateMenuFromSettings() {
    document.querySelectorAll('.menu-item').forEach((item) => setOptionsForMenu(item.id));
}

function setOptionsForMenu(menuId) {
    const modal = document.querySelector(`.modal[data-id="${menuId}"]`);

    modal.querySelectorAll('.menu-icon').forEach(function (toggle) {
        if (isInBitmask(toggle.dataset.val, settings[menuId])) {
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
}

function getMaskFromMenuID(id) {
    var value = 0;
    var modal = document.querySelector(`.modal[data-id='${id}']`);

    var options = modal.querySelectorAll('img:not(.hidden)');

    options.forEach(function (toggle) {
        value += parseInt(toggle.dataset.val);
    });

    return value;
}

function resetCurrentSubmenu() {
    var focus = document.querySelector('.modal:not(.hide)');
    if (!focus) {
        focus = document.querySelector(':focus');
    }
    var menuItem = closestClass(focus, 'menu-item');

    var key = defaults_prefix + menuItem.id;
    var section_mask = decodeURIComponent(settings.get(key));
    setSubmenuByMask(menuItem, section_mask);
}

function resetAllSubmenus() {
    // Resets all submenus to the default values
    if (confirm('Are you sure that you want to reset all menu settings to the default?')) {
        document.querySelectorAll('.menu-item').forEach(function (menuItem) {
            var key = defaults_prefix + menuItem.id;
            var mask = decodeURIComponent(settings.get(key));
            setSubmenuByMask(menuItem, mask);
        });
    }
}

function setHelpText(text) {
    // Modify the help text in the footer
    document.getElementById('help-text').innerText = text;
}

function saveDefaults() {
    if (confirm('Are you sure that you want to change the default menu settings to the current selections?')) {
        document.querySelectorAll('.menu-item').forEach(function (menuItem) {
            var key = defaults_prefix + menuItem.id;

            settings[key] = getMaskFromMenuID(menuItem.id);
        });
    }
}

function isInBitmask(val, mask) {
    // Return true if the value is in the bitmask
    return (mask & val) != 0;
}

function cycleNextTab() {
    // Cycle to the next tab
    var activeTab = document.querySelector('.tab-button.active');
    var nextTab = activeTab.nextElementSibling;
    if (!nextTab) {
        // On the last tab - set the next tab as the first tab in the list
        nextTab = document.querySelector('.tab-button');
    }
    openTab(nextTab);
}

function cyclePrevTab() {
    // Cycle to the previous tab
    var activeTab = document.querySelector('.tab-button.active');
    var prevTab = activeTab.previousElementSibling;
    if (!prevTab) {
        // On the first tab - set the next tab as the last tab in the list
        tabs = document.querySelectorAll('.tab-button');
        prevTab = tabs[tabs.length - 1];
    }
    openTab(prevTab);
}
