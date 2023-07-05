<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup


import {ref} from 'vue';
import DragHeader from './components/DragHeader.vue';

import {appWindow} from "@tauri-apps/api/window";
import {Command} from '@tauri-apps/api/shell'
import {invoke} from "@tauri-apps/api/tauri";

const resList = ref([]);
const files = ref([])
const removeFileRef = ref(true);
const folderModeRef = ref(true);

appWindow.listen("tauri://file-drop", ({event, payload}) => {


    if(folderModeRef.value){
        scanFiles(payload[0]);
    }else{
        transform(payload);
    }

});

const transform = (paths:string[])=>{
    Promise.all(paths.map(filePath=>{
        const [fileName,webpPath] = getCWebpOptions(filePath);
        return callCommand(['-q','100',fileName,'-o',webpPath]);
    })).then(res=>{
        if(removeFileRef.value){
            invoke('remove_file', {str:files.value.join(',')} ).then(res=>{
                console.log(res);
            });
        }
    })
}

const getCWebpOptions = (filePath: string) => {
    const fileName = filePath.split('\\').pop();
    const webpName = `${fileName?.split('.')[0]}.webp`;
    const webpPath = filePath.replace(fileName!, webpName);
    return [fileName, webpPath];
}

const callCommand = async (cmd: string[]) => {
    return new Promise(async resolve => {
        const command = new Command('cwebp', cmd);
        const res = await command.execute();
        resolve(res);
    })
}



const scanFiles = async (folderPath: string) => {
    const res = await invoke('scan', {name: folderPath});
    files.value = res;

    transform(files.value);
}

</script>

<template>
    <m-border>
        <m-rice-paper>
            <DragHeader/>
            <div class="container">
                <h1>drag your folder or png to here.</h1>
                <m-checkbox v-model="folderModeRef">folder mode</m-checkbox>
                <m-checkbox v-model="removeFileRef">remove .png file</m-checkbox>
                <div>
                    files:
                    <div v-for="name in files">{{ name }}</div>
                    <div>


                    </div>
                </div>

            </div>
        </m-rice-paper>
    </m-border>
</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}

.zone {
    border: 5px solid blue;
    width: 200px;
    height: 100px;
}

</style>
