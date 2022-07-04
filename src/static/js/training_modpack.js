// Polyfill for NodeList.forEach.
// Allows forEach to be called directly on a node list (return type of doucment.querySelectorAll)
if (window.NodeList && !NodeList.prototype.forEach) {
    alert('Adding polyfill for NodeList.forEach');
    NodeList.prototype.forEach = Array.prototype.forEach;
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

var settings = new Map();
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
    setSubmenusFromSettings();
}

function openTab(eventTarget) {
    var selected_tab = document.getElementById(eventTarget.id.replace('button', 'tab'));
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
    selected_tab.classList.remove('hide');
    selected_tab.querySelector('button').focus();
}

function openMenuItem(eventTarget) {
    playSound('SeWebMenuListOpen');

    var targetId = eventTarget.getAttribute('data-target');
    var modal = document.querySelector(`.modal[data-id=${targetId}]`);

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

function toggleOption(eventTarget) {
    playSound('SeSelectCheck');
    if (eventTarget.parentElement.classList.contains('single-option')) {
        selectSingleOption(eventTarget);
    } else {
        eventTarget.querySelector('img').classList.toggle('hidden');
    }
}

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
    // Valid labels:
    // SeToggleBtnFocus
    // SeToggleBtnOn
    // SeToggleBtnOff
    // SeCheckboxFocus
    // SeCheckboxOn
    // SeCheckboxOff
    // SeRadioBtnFocus
    // SeRadioBtnOn
    // SeSelectCheck
    // SeSelectUncheck
    // SeBtnDecide
    // SeTouchUnfocus
    // SeBtnFocus
    // SeKeyError
    // SeDialogOpen
    // SeWebZoomOut
    // SeWebZoomIn
    // SeWebNaviFocus
    // SeWebPointerFocus
    // SeFooterFocus
    // SeFooterDecideBack
    // SeFooterDecideFinish
    // SeWebChangeCursorPointer
    // SeWebTouchFocus
    // SeWebLinkDecide
    // SeWebTextboxStartEdit
    // SeWebButtonDecide
    // SeWebRadioBtnOn
    // SeWebCheckboxUncheck
    // SeWebCheckboxCheck
    // SeWebMenuListOpen
    if (isNx) {
        window.nx.playSystemSe(label);
    } else {
        console.log('Sound Effect: ' + label);
    }
}

function exit() {
    playSound('SeFooterDecideBack');
    setSettingsFromMenu();
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
    var regex = /[?&]([^=#]+)=([^&#]*)/g,
        match;
    while ((match = regex.exec(document.URL))) {
        settings.set(match[1], match[2]);
    }
}

function setSettingsFromMenu() {
    var section;
    var mask;
    [].forEach.call(document.querySelectorAll('.menu-item'), function (menuItem) {
        section = menuItem.id;
        mask = getMaskFromSubmenu(menuItem);
        settings.set(section, mask);
    });
}

function buildURLFromSettings() {
    var url = 'http://localhost/';
    url += '?';
    settings.forEach((val, key) => {
        url += key + '=' + String(val) + '&';
    });
    return url;
}

function selectSingleOption(e) {
    // Deselect all options in the submenu
    parent = closestClass(e, 'single-option');
    siblings = parent.querySelectorAll('.menu-icon img');
    [].forEach.call(siblings, function (sibling) {
        sibling.classList.add('hide');
    });
    e.querySelector('.menu-icon img').classList.remove('hide');
}

function setSubmenusFromSettings() {
    [].forEach.call(document.querySelectorAll('.menu-item'), function (menuItem) {
        var section = menuItem.id;
        var section_mask = decodeURIComponent(settings.get(section));
        setSubmenuByMask(menuItem, section_mask);
    });
}

function setSubmenuByMask(menuItem, mask) {
    [].forEach.call(menuItem.querySelectorAll('.modal .menu-icon img'), function (toggle) {
        if (isInBitmask(toggle.dataset.val, mask)) {
            toggle.classList.remove('hide');
        } else {
            toggle.classList.add('hide');
        }
    });

    // If no setting for a Single Option is set, select the first one
    var isSingleOption = menuItem.querySelectorAll('.modal.single-option').length != 0;
    var isAllDeselected = menuItem.querySelectorAll('.modal .menu-icon img:not(.hide)').length == 0;
    if (isSingleOption & isAllDeselected) {
        selectSingleOption(menuItem.querySelector('.modal button'));
    }
}

function getMaskFromSubmenu(menuItem) {
    var val = 0;
    [].forEach.call(menuItem.querySelectorAll('.modal img:not(.hide)'), function (toggle) {
        val += parseInt(toggle.dataset.val);
    });
    return val;
}

function resetCurrentSubmenu() {
    var focus = document.querySelector('.menu-item .modal:not(.hide)');
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
        [].forEach.call(document.querySelectorAll('.menu-item'), function (menuItem) {
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
        var key;
        var mask;
        [].forEach.call(document.querySelectorAll('.menu-item'), function (menuItem) {
            key = defaults_prefix + menuItem.id;
            mask = getMaskFromSubmenu(menuItem);
            settings.set(key, mask);
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
