var pathElement = document.getElementById("path")
var contentElement = document.getElementById("content")
var newFolderElement = document.getElementById("new-folder")
var uploadElement = document.getElementById("upload")

var wsUri = 'ws:' + window.location.host + '/ws/';
console.log('Trying to connect to: ' + wsUri);
var conn = new WebSocket(wsUri);

conn.onopen = function(e) {
    console.log('Connected.');
    if (window.location.hash) {
        var path = window.location.hash.split("#").pop();
        requestFiles(path);
    } else{
        requestFiles("");
    }
}

conn.onmessage = function(e) {
    decode(e.data);
};

conn.onclose = function() {
    console.log('Disconnected.');
    displayErrorAndReload('Disconnected','Lost connection to the server.');
    conn = null;
};

window.onhashchange = function(e) {
    var path;

    // Check if the URl even has a hash, if not reload with clear path
    if (e.newURL.indexOf('#') === -1) {
        path = '';
    } else {
        path = e.newURL.split("#").pop();
    }

    requestFiles(path);
}

newFolderElement.onclick = function(e) {
    e.preventDefault();
    displayError('Not supported', 'This feature is not implemented yet, ' + 
    'but it will be in the next version.')
}

uploadElement.onclick = function(e) {
    displayError('Not supported', 'This feature is not implemented yet, ' + 
    'but it will be in the next version.')
}
// This function sorts an array of objects
// Usage: data.sort(sort_by('key', true, parseInt));
var sort_by = function(field, reverse, primer){

   var key = primer ? 
       function(x) {return primer(x[field])} : 
       function(x) {return x[field]};

   reverse = !reverse ? 1 : -1;

   return function (a, b) {
       return a = key(a), b = key(b), reverse * ((a > b) - (b > a));
     } 
}

function requestFiles(path) {
    msg = {
        "action": "requestFilelist",
        "path": decodeURI(path)
    };

    conn.send(JSON.stringify(msg));
}

function decode(input) {
    obj = JSON.parse(input)
    path = obj.path 
    if (path != '') {path += '/'}

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
    // Sort the folder and file list
    folders.sort(sort_by('name', false, function(a){return a.toUpperCase()}));
    files.sort(sort_by('name', false, function(a){return a.toUpperCase()}));

    // Create the path navigation element
    pathOutput = '<a href="#" style="text-decoration: none" class="fas fa-home"></a> / ';
    pathList = path.split("/");
    pathList.pop()

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
        var iconClass = getIconClass(files[i].category);
        var nameHTML = '<i style="color: #007bff" class="far fa-' + iconClass + '"></i>';
        nameHTML += ' ' + files[i].name; 
        var size = files[i].size;
        var downloadHref = path + files[i].name;
        var downloadLink = '<a download href="download/' + downloadHref + '"><i class="fas fa-download"></i></a>';
        output += renderRow(nameHTML, size, downloadLink);
    }

    contentElement.innerHTML = output;
}

function getIconClass(icon_type){
    if (icon_type === "audio") { return "file-audio" }
    else if (icon_type === "archive") { return "file-archive" }
    else if (icon_type === "code") { return "file-code" }
    else if (icon_type === "default") { return "file-alt" }
    else if (icon_type === "document") { return "file-word" }
    else if (icon_type === "image") { return "file-image" }
    else if (icon_type === "presentation") { return "file-powerpoint" }
    else if (icon_type === "pdf") { return "file-pdf" }
    else if (icon_type === "spreedsheet") { return "file-excel" }
    else if (icon_type === "video") { return "file-video" }
    else { console.error("Unknown filetype: " + icon_type); return "file-alt" }
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

function displayErrorAndReload(header, message) {
    $('#errorReloadModalLabel').text(header);
    $('#errorReloadModalContent').text(message);
    $('#errorReloadModal').modal('show');
    $('#errorReloadModal').on('hide.bs.modal', function (e) {
        window.location.reload();
    })
}
