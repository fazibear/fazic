var Module = {
  print: function(text) { console.log(text) },
  printErr: function(text) { console.error(text) },
  canvas: document.getElementById('canvas'),
  setStatus: function(status) {
    document.getElementById('status').innerHTML = status;
  }
};
Module.setStatus('Downloading...');
