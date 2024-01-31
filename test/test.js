"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var index_1 = __importDefault(require("../lib/index"));
var x = index_1.default.getComments(["D:\\@Download\\test\\a.mp4", "D:\\@Download\\test\\b.mp4"]);
console.log(x);
var oldComment = x["D:\\@Download\\test\\b.mp4"];
var newComment = "test";
index_1.default.setComment("D:\\@Download\\test\\b.mp4", newComment);
var y = index_1.default.getComments(["D:\\@Download\\test\\b.mp4"]);
console.log(y);
console.log(y["D:\\@Download\\test\\b.mp4"] == newComment);
index_1.default.setComment("D:\\@Download\\test\\b.mp4", oldComment);
var z = index_1.default.getComments(["D:\\@Download\\test\\b.mp4"]);
console.log(z);
console.log(z["D:\\@Download\\test\\b.mp4"] == oldComment);
try {
    index_1.default.setComment("D:\\@Download\\test\\b.mp4", oldComment, "");
}
catch (ex) {
    console.log(ex);
}
