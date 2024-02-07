import metadata from "../lib/index";
import { files } from "./file"

const runTest = async () => {

    let x = await metadata.getValues(files, "Comment");
    console.log(x);

    const all = await metadata.read(files[1]);
    console.log(all)

    const target = files[1];
    const newComment = String(new Date().getTime())

    await metadata.setValue(target, "Comment", newComment);

    x = await metadata.getValues(files, "Comment");
    console.log(x);

    // try{
    //     //@ts-ignore
    //     await metadata.setComment(target, oldComment, "");
    // }catch(ex:any){
    //     console.log(ex.message)
    // }

}

runTest();