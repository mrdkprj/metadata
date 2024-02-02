import metadata from "../lib/index";
import { files } from "./file"

const x = metadata.getComments(files);
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