--!strict
local fs = require("@std/fs")
local env = require("@std/env")
local time = require("@std/time")
local format = require("@std/io/format")
local http = require("@std/net/http")
local process = require("@std/process")

if env.os ~= "Linux" then
    print("You can run this test manually if you have uv (python) installed; \z
    i couldn't figure out a good way to bootstrap uv on platforms outside Linux")
    process.exit()
end

--[[
    For now, this test uses python and the uv package manager for python
    because it's a good way to test binary response w/ rfc compliant multipart file upload
]]

local function install_uv()
    local probe_ok, probe_result = pcall(process.run, {
        program = "uv",
        args = { "--help" }
    })
    if probe_ok and typeof(probe_result) ~= "error" and probe_result.ok then
        print("uv already installed")
        return -- don't install uv if already installed
    end

    -- gh actions runners always have python; use pip to bootstrap uv on any OS
    local python_cmd = if env.os == "Windows" then "python" else "python3"
    
    process.run {
        program = python_cmd,
        args = { "-m", "pip", "install", "uv", "--quiet" },
    }:unwrap()
    
end

install_uv()

local python_code = [[
from flask import Flask, request, send_file, jsonify
import os

app = Flask(__name__)
UPLOAD_FOLDER = "./uploads"
os.makedirs(UPLOAD_FOLDER, exist_ok=True)

@app.route("/upload", methods=["POST"])
def upload_file():
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    file = request.files["file"]
    if file.filename == "":
        return jsonify({"error": "No file selected"}), 400
    filepath = os.path.join(UPLOAD_FOLDER, file.filename)
    file.save(filepath)
    return jsonify({"message": "File uploaded successfully", "filename": file.filename}), 201

@app.route("/download/<filename>", methods=["GET"])
def download_file(filename):
    filepath = os.path.join(UPLOAD_FOLDER, filename)
    if not os.path.exists(filepath):
        return jsonify({"error": "File not found"}), 404
    return send_file(filepath, as_attachment=True)

@app.route("/", methods=["GET"])
def home():
    return jsonify({"message": "Dummy REST API with binary support is running"}), 200

if __name__ == "__main__":
    app.run(debug=True, use_reloader=False, port=5150)
]]

local python_project_dir = fs.path.join(script:parent(), "dummy_python_upload_api")
local function start_flask_server()
    -- clean up any leftover projects from a previous failed run
    fs.dir.try_remove(python_project_dir)

    -- init uv project in script:parent()
    process.run {
        program = "uv",
        args = { "init", python_project_dir },
    }:unwrap()

    -- uv add flask
    process.run {
        program = "uv",
        args = { "add", "flask" },
        cwd = python_project_dir,
    }:unwrap()

    fs.writefile(fs.path.join(python_project_dir, "main.py"), python_code)

    local server = process.spawn {
        program = "uv",
        args = { "run", "main.py" },
        cwd = python_project_dir,
    } :: process.PipedChild

    print("Starting python flask server")

    return server
end

local server = start_flask_server()

local ok, err = pcall(function()

    -- Flask debug info goes to stderr; wait until it says it's listening
    local startup = server.stderr:read_to("Running on", true, 5)
    if not startup then
        print(server.stderr:read())
    end
    assert(startup ~= nil, "Flask server did not start within 5 seconds")

    -- Give the server a moment to fully bind before sending requests
    time.wait(1)

    local filename = "nanuk.png"
    local original_data = fs.readfile(fs.path.join(".", "tests", "data", filename))

    -- Upload the image via multipart/form-data
    local function upload()
        local boundary = "BOUNDARRRY123"
        local multipart_body = table.concat({
            "--" .. boundary,
            `Content-Disposition: form-data; name="file"; filename="{filename}"`,
            "Content-Type: application/octet-stream",
            "",
            original_data,
            "--" .. boundary .. "--",
        }, "\r\n")

        local response = http.post {
            url = "http://127.0.0.1:5150/upload",
            headers = {
                ["Content-Type"] = `multipart/form-data; boundary={boundary}`,
                ["Content-Length"] = `{#multipart_body}`,
            },
            body = buffer.fromstring(multipart_body),
        }

        assert(response.ok, `Upload failed: {response.status.code} {response.status.reason}: {response.body}`)
        assert(response.status.code == 201, `Expected 201 Created, got {response.status.code}`)
        -- print(response)
    end

    upload()

    -- Download and verify the binary response
    local downloaded = http.get {
        url = `http://127.0.0.1:5150/download/{filename}`,
    }

    assert(downloaded.ok, `Download failed: {downloaded.status.code} {downloaded.status.reason}`)
    assert(
        downloaded.content_type ~= nil and downloaded.content_type.mime_type ~= nil,
        "Downloaded response should have a content_type"
    )

    local mime_type = downloaded.content_type.mime_type
    assert(
        mime_type == "image/png" or mime_type == "image/jpeg" or mime_type == "application/octet-stream",
        `Unexpected content-type mime_type: {format(downloaded.content_type)}`
    )

    assert(
        #downloaded.body == #original_data,
        `Downloaded file size {#downloaded.body} doesn't match original {#original_data}`
    )

    assert(
        downloaded.body == original_data,
        "Downloaded file content does not match the original uploaded file"
    )
    return nil
end) -- end pcall

-- Always clean up, even if the test failed
server:kill()

-- Flask's debug mode spawns a child reloader process that outlives the parent;
-- kill any leftover process still bound to port 5150
local lsof = process.run {
    program = "lsof",
    args = { "-ti:5150" },
}
if lsof.ok then
    local pid = lsof.stdout:match("(%d+)")
    if pid then
        local killed = process.run {
            program = "kill",
            args = { pid },
        }
        if killed.ok then
            print(`killed process with pid {pid} hogging port 5150`)
        end
    end
end

fs.dir.try_remove(python_project_dir)

if not ok then
    error(err)
end

print("binary upload/download test passed")
