var isNx = (typeof window.nx !== 'undefined')

var isPageSwithed = false
var isSoundLoaded = !isNx
var isLoadedFont = false
var isBgAnimation = false
var CTRL_KEY = 'ctrl'

function saveCtrlKey() {
  // セッションストレージにコントロールデータを入れる
  if (!window.sessionStorage.getItem(CTRL_KEY)) {
    var queryStrs = decodeURIComponent(window.location.search).split('?')
    if (queryStrs.length === 2) {
      var queryArr = queryStrs[1].split('&')
      queryArr.forEach(function(query) {
        var kv = query.split('=')
        var ctrlValue = kv[1] ? kv[1] : ''
        if (kv.length === 2 && kv[0] === CTRL_KEY && kv[1]) {
          // 整数値
          sessionStorage.setItem(CTRL_KEY, parseInt(ctrlValue, 10))
        }
      })
    }
  }
}

saveCtrlKey()

function canFocus() {
  return !isPageSwithed
}

function canPageSwitch() {
  return !isPageSwithed && isSoundLoaded
}

function shouldShowAfterFontLoadingElm() {
  return isLoadedFont && !isBgAnimation
}

function incrementPlayReportCounter(arg) {
  if (isNx && window.nx.playReport) {
    if (typeof arg === 'number') {
      window.nx.playReport.incrementCounter(arg)
    } else if (arg.dataset && arg.dataset.counterId) {
      var counterId = parseInt(arg.dataset.counterId, 10)
      window.nx.playReport.incrementCounter(counterId)
    }
  }
}

function playSound(label, opt_onEnded) {
  if (isNx && window.wsnd && window.wsnd.play) {
    if (isSoundLoaded) {
      window.wsnd.play(label, opt_onEnded)
    } else if (opt_onEnded) {
      opt_onEnded()
    }
  } else if (opt_onEnded) {
    setTimeout(function() {
      opt_onEnded()
    }, 500)
  }
}

function disabledOtherLink(selectedId) {
  $('a').each(function() {
    var otherId = $(this).attr('id')
    if (typeof otherId === 'undefined' || typeof selectedId === 'undefined' || otherId !== selectedId) {
      $(this).attr('tabindex', -1)
    }
  })
}

function fadeOutPage(animationEndFunc) {
  setTimeout(function() {
    // 黒背景
    $('body').prepend('<div id="fade-mask" class="fade-mask"></div>')
    if (animationEndFunc) {
      $('#fade-mask').on('webkitAnimationEnd', animationEndFunc)
    }
  }, 83)
}

function switchPage(e, url, counterId) {
  // ページ遷移中ならなにもしない(連打対策)
  if (!canPageSwitch()) return

  isPageSwithed = true
  // 遷移中はどこにも動けないようにする
  disabledOtherLink(e.id)

  var incrementCounterId = (typeof counterId === 'undefined') ? e : counterId
  incrementPlayReportCounter(incrementCounterId)
  var nextPath = url || $(e).attr('ref')

  // 音を鳴らす
  playSound('fixed', function() {
    // urlが指定されているならurlを優先
    window.location.href = nextPath
  })

  // フェードアウト
  fadeOutPage()
}

function goBack() {
  // ページ遷移中ならなにもしない(連打対策)
  if (!canPageSwitch()) return

  isPageSwithed = true
  $('.is-focused').addClass('is-pause-anim')
  $('#ret-button').addClass('is-focus')
  // 遷移中はどこにも動けないようにする
  disabledOtherLink()

  // 音を鳴らす
  playSound('cancel')

  // フェードアウト
  fadeOutPage(function() {
    window.history.back()
  })
}

function appendFontCss() {
  if (!$('#font-stylesheet')[0]) {
    $('head').append('<link id="font-stylesheet" rel="stylesheet" href="../../css/font.css">')
  }
}

function loadFont() {
  appendFontCss()
  document.fonts.addEventListener('loadingdone', function(e) {
    isLoadedFont = true
    if (shouldShowAfterFontLoadingElm()) {
      // 正常に読み込まれた場合の処理
      $('.show-after-font-loading').addClass('is-appear')
      $('#pre-font-loading').removeClass('is-hidden')
    }
  })
}

function disableTouch() {
  // aタグ以外タッチを効かないようにする
  window.addEventListener('touchstart', function(e) {
    if (e.target.tagName !== 'A' && $(e.target).parents('a').length === 0) {
      e.preventDefault()
    }
  }, {
    once: false,
    passive: false,
    capture: true
  })
}

function loadImage() {
  function replaceImg(e) {
    // img srcに変更
    e.bind('load', function() {
      e.addClass('is-appear')
    })
    e.attr('src', e.attr('ref'))
  }
  $('img').each(function() {
    var $thisObj = $(this)
    if (typeof $thisObj.attr('src') === 'undefined') {
      var ref = $thisObj.attr('ref')
      // replace to SVG
      if (typeof ref === 'string' && ref.match(/.svg$/)) {
        $.ajax({
          type: 'GET',
          url: ref,
          success: function(data) {
            var $svg = $(data)
            var classNames = $thisObj.attr('class')
            $svg.addClass(classNames)
            $svg.addClass('is-appear')
            $thisObj.replaceWith($svg)
          },
          error: function() {
            replaceImg($thisObj)
          }
        })
        return
      }
      replaceImg($thisObj)
    }
  })
}

$(function() {
  if (!isNx) {
    // 画像のロード
    setTimeout(loadImage, 0)
  }
})

if (isNx) {
  // 音の設定
  if (window.wsnd) {
    window.wsnd.load({
      'cursor': '../../../common/audio/se_system_cursor.wav',
      'cancel': '../../../common/audio/se_system_cancel.wav',
      'fixed': '../../../common/audio/se_system_fixed.wav'
    }, function() {
      isSoundLoaded = true
    })
  } else {
    isSoundLoaded = true
  }
  // フッターの設定
  window.nx.footer.setAssign('B', '', goBack, {
    se: ''
  })
  window.nx.footer.unsetAssign('X')
  window.addEventListener('NXFirstPaintEndAfterLoad', function() {
    // 画像のロード
    setTimeout(loadImage, 0)
  })
  // プレイレポート
  if (window.nx.playReport) {
    // 改訂があった場合は数字を変更して下さい
    window.nx.playReport.setCounterSetIdentifier(0)
  }
}
