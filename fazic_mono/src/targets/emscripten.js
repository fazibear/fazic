mergeInto(LibraryManager.library, {
  js_fetch: function(name_p, method_p, data_p, code_p, resp_p, size_p) {
    var name = Pointer_stringify(name_p);
    var method = Pointer_stringify(method_p);
    var data = Pointer_stringify(data_p) || null;

    var code = 200;
    var response;

    var bas_ext = ".BAS"
    var filename = function(name) {
      return name.toUpperCase() + bas_ext;
    }

    switch(method) {
      case "dir":
        response = ""
        for (var i = 0, len = localStorage.length; i < len; ++i) {
          var file = localStorage.key(i);
          if(file.endsWith(bas_ext)) {
            response = response + 'LOAD "' + file.substr(0, file.length - 4) + '"' + "\n";
          }
        }
        break;
      case "load":
        response = localStorage.getItem(filename(name));
        if(!response){
          response = "NOT FOUND";
          code = 404;
        }
        break;
      case "save":
        localStorage.setItem(filename(name), data);
        response = "SAVED";
        break;
    }

    stringToUTF8(response, resp_p, 102400);

    var size = new Blob([response]).size;

    setValue(size_p, size, 'i32');
    setValue(code_p, code, 'i32');
  },
});
