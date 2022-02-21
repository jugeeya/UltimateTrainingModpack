var $prevQuestionMsg = null
var $prevFocusedElm = null
var isStopCursorAnim = false
var isScrolling = false
var isFocusWaiting = false
var reportedCounterIds = {}
var scrollTimeoutId = null
var disableSound = true
var cursorSoundCount = 0

var OFFSET_Y = 85
var CURSOR_WIDTH = 980
var CURSOR_BACKGROUND_WIDTH = CURSOR_WIDTH * 2
var CURSOR_VELOCITY = 2.8
var FOCUS_WAIT_TIME = 300
var MAX_SCROLL_TIME = 480
var MIN_SCROLL_TIME = 300
var SCROLL_TIME_RATIO = 0.78
var SCROLL_FOCUS_WAIT_TIME = 270
var SCROLL_AFTER_WAIT_TIME = 200
var MAX_SOUND_COUNT = 5

function toggleAnswer(qaId) {
  function convertQAtoAId(focusedId) {
    return 'answer-border' + focusedId.replace(/qa-/g, '-')
  }
  var answerId = convertQAtoAId(qaId)
  var $qaElm = $('#' + qaId)
  $qaElm.toggleClass('is-opened')
  $('#' + answerId).toggleClass('is-hidden')

  return $qaElm.hasClass('is-opened')
}

function getScrollTop(qaId) {
  var $qaElm = $('#' + qaId)
  var qaPos = $qaElm.offset().top
  return qaPos - OFFSET_Y
}

function scrollQA(qaId, endFunc) {
  isScrolling = true
  isStopCursorAnim = true
  clearTimeout(scrollTimeoutId)
  scrollTimeoutId = null

  var scrollTop = getScrollTop(qaId)

  var windowScrollTop = $(window).scrollTop()
  // スクロールしない場合
  if (scrollTop === windowScrollTop) {
    isScrolling = false
    isStopCursorAnim = false
    return
  }

  var duration = Math.abs(scrollTop - windowScrollTop) * SCROLL_TIME_RATIO
  duration = Math.min(Math.max(duration, MIN_SCROLL_TIME), MAX_SCROLL_TIME)

  $("html,body").stop().animate({
    scrollTop: scrollTop
  }, duration, 'swing', function() {
    if (endFunc) {
      requestAnimationFrame(endFunc)
    }
    scrollTimeoutId = setTimeout(function() {
      isStopCursorAnim = false
    }, SCROLL_AFTER_WAIT_TIME)

    isScrolling = false
  })
}

function openAnswer(e) {
  if (!canPageSwitch() || disableSound) return

  var isOpened = toggleAnswer(e.id)
  if (isOpened) {
    scrollQA(e.id)
    // 音を鳴らす
    playSound('fixed')
    // プレイレポート送信
    var counterId = parseInt(e.dataset.counterId, 10)
    if (typeof reportedCounterIds[counterId] !== 'undefined') return
    reportedCounterIds[counterId] = true
    incrementPlayReportCounter(counterId)
  } else {
    playSound('cursor')
  }
}

function focusQA(e) {
  if (!canFocus()) return

  isFocusWaiting = true

  if (!disableSound) {
    if (cursorSoundCount < MAX_SOUND_COUNT) {
      playSound('cursor', function() {
        cursorSoundCount = cursorSoundCount - 1
      })
      cursorSoundCount = cursorSoundCount + 1
    }
  }

  var qaId = e.id

  function convertQAtoQId(focusedId) {
    return 'question' + focusedId.replace(/qa-/g, '-')
  }

  var qId = convertQAtoQId(e.id)
  // 袋文字の追加
  var $firstMsg = $('#' + qId + ' .question-message span:first')
  $prevQuestionMsg = $firstMsg.clone()
  $firstMsg.after($prevQuestionMsg)

  // is-focused classの追加
  $prevFocusedElm = $('#' + qaId)
  $prevFocusedElm.addClass('is-focused')
}

function defocusQA(e) {
  if (!canFocus()) return
  isFocusWaiting = true
  if ($prevFocusedElm) {
    // is-focused classの削除
    $prevFocusedElm.removeClass('is-focused')
    if ($prevFocusedElm.css('background-position-x') !== '0%') {
      $prevFocusedElm.css('background-position-x', "0%")
    }
    $prevFocusedElm = null
  }
  if ($prevQuestionMsg) {
    $prevQuestionMsg.remove()
    $prevQuestionMsg = null
  }
}

$(function() {
  disableTouch()

  var qaId = 'qa' + window.location.hash.replace('#', '-')

  var $qElm = $('#' + qaId)
  if ($qElm[0]) {
    toggleAnswer(qaId)
    window.scroll(0, getScrollTop(qaId))
    setTimeout(function() {
      $('.l-one-column').css('opacity', 1)
      $qElm[0].focus()
      disableSound = false
    }, 200)
    // キーワードから来た場合は、再度送信しない
    var counterId = parseInt($qElm.attr('data-counter-id'), 10)
    reportedCounterIds[counterId] = true
  } else {
    $('.l-one-column').css('opacity', 1)
    disableSound = false
  }

  var focusTimeoutId = null
  window.addEventListener('keydown', function(e) {
    if (!(e.keyCode === 40 || e.keyCode === 38)) return

    var isDefault = false
    // タイムアウトのクリア
    if (focusTimeoutId) {
      clearTimeout(focusTimeoutId)
      focusTimeoutId = setTimeout(function() {
        focusTimeoutId = null
      }, SCROLL_FOCUS_WAIT_TIME)
      isDefault = true
    }

    // スクロールアニメーション中にスクロールしようとしたら、スクロールアニメーションを止める
    if (isScrolling) {
      $("html,body").stop()
      isScrolling = false
      isStopCursorAnim = false
      isDefault = true
    }

    if (isDefault) return

    var MAX_QA_NUM = 20
    var currentTabIndex = document.activeElement.tabIndex
    if (e.keyCode === 40) { // down
      var nextTabIndex = currentTabIndex + 1
      if (nextTabIndex <= MAX_QA_NUM) {
        var $nextElm = $("[tabindex=" + nextTabIndex + "]")
        if ($nextElm.hasClass('is-opened')) {
          // 下端が画面外なら、スクロールさせる
          var nextElmBottom = $nextElm.offset().top + $nextElm.height()
          var windowBottom = $(window).scrollTop() + window.innerHeight
          if (nextElmBottom > windowBottom) {
            e.preventDefault()
            var qaId = $nextElm.attr('id')
            focusTimeoutId = setTimeout(function() {
              scrollQA(qaId, function() {
                $nextElm[0].focus()
                focusTimeoutId = null
              })
            }, SCROLL_FOCUS_WAIT_TIME)
          }
        }
      }
    } else if (e.keyCode === 38) { // up
      var nextTabIndex = currentTabIndex - 1
      if (nextTabIndex >= 0) {
        var $nextElm = $("[tabindex=" + nextTabIndex + "]")
        if ($nextElm.hasClass('is-opened')) {
          // 上端が画面外なら、スクロールさせる
          var nextElmTop = $nextElm.offset().top
          var windowTop = $(window).scrollTop() + OFFSET_Y
          if (nextElmTop < windowTop) {
            e.preventDefault()
            var qaId = $nextElm.attr('id')
            focusTimeoutId = setTimeout(function() {
              scrollQA(qaId, function() {
                $nextElm[0].focus()
                focusTimeoutId = null
              })
            }, SCROLL_FOCUS_WAIT_TIME)
          }
        }
      }
    }
  }, {
    once: true,
    passive: true,
    capture: true
  })

  var prevTime = 0
  var cursorPosX = 0
  var prevFocusTime = 0

  function update(timestamp) {
    var now = timestamp
    if (isFocusWaiting) {
      prevFocusTime = now
      isFocusWaiting = false
      cursorPosX = 0
    }
    if (!isScrolling && !isStopCursorAnim && !isPageSwithed && $prevFocusedElm && !isFocusWaiting && (now - prevFocusTime) > FOCUS_WAIT_TIME) {
      var diff = now - prevTime
      cursorPosX = (cursorPosX + Math.floor(CURSOR_VELOCITY * diff)) % CURSOR_BACKGROUND_WIDTH
      $prevFocusedElm.css('background-position-x', cursorPosX + "px")
    }
    prevTime = now
    requestAnimationFrame(update)
  }
  // update
  requestAnimationFrame(update)
})
