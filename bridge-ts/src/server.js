"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var express_1 = __importDefault(require("express"));
var port = 8080;
var host = "localhost";
function startServer(chainApi) {
    var app = express_1.default();
    app.get("/status", function (req, res) {
        //
    });
    app.listen(port, host, function () {
        console.log("Listening on http://" + host + ":" + port);
    });
}
exports.startServer = startServer;
