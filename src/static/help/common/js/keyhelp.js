/* ボタン表記取得・変更クラス        */
/* 更新日 2018/6/5                */
/* 作成者 Akinori Kamiya(BNS)     */

/**
 * ボタン表記取得クラス
 * @constructor
 * @param   {String} controller コントローラのタイプ
 */
var controller_setting = function (controller) {
  // 0: 本体接続
  // 1: Joycon2本持ち
  // 2: Joycon横
  // 3: PROコン
  // 4: GCコン
  
  var my_controller = Number(controller);
  if (my_controller === null) {
    my_controller = 0;
  }
  var keylist = ['A', 'B', 'X', 'Y', 'L', 'R', 'ZL', 'ZR', '+', '-', 'HOME'];
  this.get_keycodeset = function (ctrl) {
    var keycode_set = [];
    switch (ctrl) {
    case 0:
      keycode_set = ["e0e0", "e0e1", "e0e2", "e0e3", "e0e4", "e0e5", "e0e6", "e0e7", "e0f1", "e0f2", "e0f4"];
      break;
    case 1:
      keycode_set = ["e0e0", "e0e1", "e0e2", "e0e3", "e0e4", "e0e5", "e0e6", "e0e7", "e0ef", "e0f0", "e0f4"];
      break;
    case 2:
      keycode_set = ["e0ab", "e0ac", "e0ad", "e0ae", "e0e8", "e0e9", "e0e8", "e0e9", "e0f1", "e0f2", "e0f4"];
      break;
    case 3:
      keycode_set = ["e0e0", "e0e1", "e0e2", "e0e3", "e0e4", "e0e5", "e0e6", "e0e7", "e0f1", "e0f2", "e0f4"];
      break;
    case 4:
      keycode_set = ["e0e0", "e0e1", "e206", "e207", "e204", "e205", "e204", "e205", "e0f1", "e0f2", "e0f4"];
      break;
    }
    return keycode_set;
  };
  /**
   * コントローラに対しての特定のキー表記文字を取得する
   * @param   {String} key 取得したいキー文字
   * @returns {String} 文字を返す 
   */
  this.get_string = function (key) {
    var keycode_set = this.get_keycodeset(my_controller);
    var num = keylist.indexOf(key);
    var return_value;
    if (num === -1) {
      return_value = key;
    } else {
      return_value = String.fromCodePoint("0x" + keycode_set[num]);
    }
    return return_value;
  };
  /**
   * HTMLDocumnet内で指定DOMクラスのdata-btnカスタム属性の値を、取得したキーコードに変更する
   * @param {String} classname 変換を適用させたいDOMクラス名
   */
  this.set_document_keycode = function (classname) {
    var keycode_set = this.get_keycodeset(my_controller);
    var itm = [].slice.call(document.querySelectorAll('[class^=' + classname + ']'), 0);
    itm.forEach(function (val) {
      val.setAttribute('data-btn', String.fromCodePoint("0x" + keycode_set[keylist.indexOf(val.getAttribute('data-btn'))]) + " ");
    });
  };
};
