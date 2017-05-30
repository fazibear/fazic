mergeInto(LibraryManager.library, {
  js_fetch: function(url_p, method_p, data_p, code_p, resp_p, size_p) {
    var url = Pointer_stringify(url_p);
    var method = Pointer_stringify(method_p);
    var data = Pointer_stringify(data_p) || null;

    var code, response;

    try {
      var request = new XMLHttpRequest();
      request.open(method, url, false);
      request.send(data);
      code = request.status;
      response = request.responseText;
    } catch(err) {
      code = 0;
      response = "CAN'T CONNECT"
    }

    var size = new Blob([response]).size + 1;
    stringToUTF8(response, resp_p, size);

    setValue(size_p, size, 'i32');
    setValue(code_p, code, 'i32');
  },
});
