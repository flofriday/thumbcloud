var pathElement = document.getElementById("path")
var contentElement = document.getElementById("content")

var wsUri = 'ws:' + window.location.host + '/ws/';
console.log('Trying to connect to: ' + wsUri);
var conn = new WebSocket(wsUri);

conn.onopen = function(e) {
    console.log('Connected.');
    if (window.location.hash) {
        var path = window.location.hash.split("#").pop();
        requestFiles(path);
    } else{
        requestFiles(".");
    }
}

conn.onmessage = function(e) {
    decode(e.data);
};

conn.onclose = function() {
    console.log('Disconnected.');
    displayError('Disconnected','Lost connection to the server.');
    conn = null;
};

window.onhashchange = function(e) {
    var path = e.newURL.split("#").pop();
    requestFiles(path);
}

function requestFiles(path) {
    msg = {
        "action": "requestFilelist",
        "path": path
    };

    conn.send(JSON.stringify(msg));
    console.log(JSON.stringify(msg));
}

function decode(input) {
    obj = JSON.parse(input)
    path = obj.path + "/";

    if (obj.action == 'sendFilelist') {
        renderFiles(path, obj.folders, obj.files);

    } else if (obj.action == 'sendError') {
        console.log('Got Error from Server:' + obj.message);
        displayError('Internal Server Error', obj.message);

    } else {
        console.log('Got invalid "' + obj.action + '" as action from sever')
    }
}

function renderFiles(path, folders, files) {
    // Create the path navigation element
    if (path != '') {
        pathOutput = '';
        pathList = path.split("/");
        console.log(pathList);

        for (i = 0; i < pathList.length; i++) {
            var fullPath  = ""; 
            for (j = 0; j <= i; j++) {
                fullPath += pathList[j];
                if (j != i) {
                    fullPath += '/';
                }
            }
            pathOutput += '<a href="#' + fullPath + '">' + pathList[i] + '</a> / ';
        }

        pathOutput = pathOutput.substring(0, pathOutput.length - 2);
        pathElement.innerHTML = '<h5>' + pathOutput + '</h5>';
    } 

    // Create the file and folder list
    var output = '';

    // Check if there are even elements in that folder
    if (folders.length == 0 && files.length == 0) {
        output += renderRow('<i>this folder is empty</i>', '', '');
    }

    // Render folders
    for (i = 0; i < folders.length; i++) {
        var name = folders[i].name;
        var icon = '<i style="color: #007bff" class="fas fa-folder"></i> ';
        var nameHTML = '<a href="#' + path + name + '" >' + icon + name + "</a>";
        output += renderRow(nameHTML, '', '');
    }

    // Render files
    for (i = 0; i < files.length; i++) {
        var nameHTML = '<i style="color: #007bff" class="far fa-file"></i>';
        nameHTML += ' ' + files[i].name; 
        var size = files[i].size;
        var downloadHref = path + files[i].name;
        var downloadLink = '<a target=”_blank” href="download/' + downloadHref + '"><i class="fas fa-download"></i></a>';
        output += renderRow(nameHTML, size, downloadLink);
    }

    contentElement.innerHTML = output;
}

function renderRow(name, size, download) {
    var out = '<tr><td>'+name+'</td><td>'+size+'</td><td>' + download + '</td></tr>';
    return out;
}

function displayError(header, message) {
    $('#errorModalLabel').text(header);
    $('#errorModalContent').text(message);
    $('#errorModal').modal('show');
}

