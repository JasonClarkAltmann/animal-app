<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { AnimalData } from "../interfaces/AnimalData";
import { Bookmark, Hourglass, RefreshCcw, Weight } from "lucide-vue-next";
import { Button, Image, Skeleton } from "primevue";

const catData = ref<AnimalData[]>([]);

const loading = ref(false);
const error = ref<Error | null>(null);

const fetchCatData = async () => {
  try {
    loading.value = true;
    error.value = null;

    const result = await invoke("fetch_cat_data");

    catData.value = result as AnimalData[];
  } catch (err) {
    error.value = err as Error;
  } finally {
    loading.value = false;
  }
};

onMounted(fetchCatData);
</script>

<template>
  <div class="flex flex-col gap-8 p-4">
    <div class="flex flex-col flex-grow justify-center items-center">
      <div v-if="loading">
        <Skeleton width="20rem" height="20rem"></Skeleton>
      </div>
      <div v-else-if="error">
        Fehler beim Laden der Bilder: {{ error.message || error }}
      </div>
      <div v-else-if="catData.length === 0">Keine Katzenbilder gefunden</div>
      <div
        v-else
        class="flex flex-col gap-8 justify-center items-center font-bold"
      >
        <div class="flex flex-row gap-2 text-3xl">
          <Bookmark :size="35" />
          {{ catData[0].breeds[0]?.name || "N/A" }}
        </div>

        <Image :src="catData[0].url" alt="Katzenbild" width="400" preview />

        <div class="flex w-full justify-between gap-2">
          <div class="flex flex-row gap-2 text-xl">
            <Weight :size="28" />
            {{ catData[0].breeds[0].weight.metric + " kg" || "N/A" }}
          </div>
          <div class="flex flex-row gap-2 text-xl">
            <Hourglass :size="28" />
            {{ catData[0].breeds[0]?.life_span + " Jahre" || "N/A" }}
          </div>
        </div>
      </div>
    </div>

    <div class="flex justify-center mt-auto">
      <Button label="Neues Bild" @click="fetchCatData" fluid>
        <template #icon>
          <RefreshCcw />
        </template>
      </Button>
    </div>
  </div>
</template>
