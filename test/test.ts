import metadata from "../lib/index";
import { files } from "./file"

const runTest = async () => {
    const x = await metadata.getComments(files);
    console.log(x);

    const target = files[1];
    const oldComment = x[target]
    const newComment = "test"
/*
    await metadata.setComment(target, newComment);

    const y = await metadata.getComments([target]);
    console.log(y)
    console.log(y[target] == newComment)

    await metadata.setComment(target, oldComment);

    const z = await metadata.getComments([target]);
    console.log(z)
    console.log(z[target] == oldComment)

    try{
        //@ts-ignore
        await metadata.setComment(target, oldComment, "");
    }catch(ex:any){
        console.log(ex.message)
    }*/

    const all = await metadata.read(files[0]);
    console.log(all)
}

runTest();