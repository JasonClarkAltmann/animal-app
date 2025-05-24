<script setup lang="ts">
import { ref } from "vue";
import CatPanel from "../components/CatPanel.vue";
import DogPanel from "../components/DogPanel.vue";
import {
  Tabs,
  TabPanels,
  TabPanel,
  Panel,
  Button,
  ButtonGroup,
  Divider,
} from "primevue";
import { Cat, Dog } from "lucide-vue-next";

const activeTab = ref("0");
</script>

<template>
  <Panel class="h-full w-full sm:h-fit sm:w-fit p-4 overflow-auto">
    <template #header>
      <div class="flex flex-col w-full justify-center items-center">
        <h2 class="text-4xl font-bold">Jasons Tierbilder</h2>
        <Divider />
      </div>
    </template>
    <ButtonGroup class="w-full">
      <Button
        label="Hunde"
        @click="activeTab = '0'"
        :outlined="activeTab !== '0'"
        class="w-full"
        rounded
      >
        <template #icon>
          <Dog />
        </template>
      </Button>
      <Button
        label="Katzen"
        @click="activeTab = '1'"
        :outlined="activeTab !== '1'"
        class="w-full"
        rounded
      >
        <template #icon>
          <Cat />
        </template>
      </Button>
    </ButtonGroup>
    <Tabs v-model:value="activeTab">
      <TabPanels>
        <TabPanel value="0">
          <DogPanel />
        </TabPanel>
        <TabPanel value="1">
          <CatPanel />
        </TabPanel>
      </TabPanels> </Tabs
  ></Panel>
</template>
