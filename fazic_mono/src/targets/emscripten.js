mergeInto(LibraryManager.library, {
  js_fetch: function(url_p, method_p, data_p, code_p, resp_p, size_p) {
    url = Pointer_stringify(url_p);
    method = Pointer_stringify(method_p);
    data = Pointer_stringify(data_p) || null;

    var request = new XMLHttpRequest();
    request.open(method, url, false);
    request.send(data);

    var str = request.responseText;
    var code = request.status;

    var size = new Blob([str]).size + 1;
    stringToUTF8(str, resp_p, size);

    setValue(size_p, size, 'i32');
    setValue(code_p, code, 'i32');

    console.log(str);
  },
});
