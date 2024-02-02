"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var index_1 = __importDefault(require("../lib/index"));
var file_1 = require("./file");
var x = index_1.default.getComments(file_1.files);
console.log(x);
/*
const target = files[1];
const oldComment = x[target]
const newComment = "test"

metadata.setComment(target, newComment);

const y = metadata.getComments([target]);
console.log(y)
console.log(y[target] == newComment)

metadata.setComment(target, oldComment);

const z = metadata.getComments([target]);
console.log(z)
console.log(z[target] == oldComment)

try{
    //@ts-ignore
    metadata.setComment(target, oldComment, "");
}catch(ex:any){
    console.log(ex.message)
}
*/ 
