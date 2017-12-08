mergeInto(LibraryManager.library, {
  js_fetch: function(name_p, method_p, data_p, code_p, resp_p, size_p) {
    var name = Pointer_stringify(name_p);
    var method = Pointer_stringify(method_p);
    var data = Pointer_stringify(data_p) || null;

    var code = 200;
    var response;

//    try {
      switch(method) {
        case "load":
          response = localStorage.getItem(name + ".bas");
          break;
        case "save":
          localStorage.setItem(name + ".bas", data);
          response = "SAVED";
          break;
      }
  //  }
    //catch(err) {
    //  response = "ERROR"
    //}

    stringToUTF8(response, resp_p, 102400);

    var size = new Blob([response]).size;

    setValue(size_p, size, 'i32');
    setValue(code_p, code, 'i32');
  },
});
