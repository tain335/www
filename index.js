var express = require('express');
var app = express();
var fs = require('fs');
var path = require('path');

fs.readdir(__dirname, function(err, files) {
    for(let file of files) {
        if(!(/node_modules|^\./.test(file)) && fs.statSync(path.join(__dirname, file)).isDirectory()) {
            app.use('/' + file, express.static(file));
        }
    }
});

app.listen(8080);