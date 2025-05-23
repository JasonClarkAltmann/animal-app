<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { DogData } from "../interfaces/DogData";
import { RefreshCcw } from "lucide-vue-next";
import Button from "primevue/button";
import Image from "primevue/image";
import Card from "primevue/card";
import Skeleton from "primevue/skeleton";
import { Divider } from "primevue";

const dogData = ref<DogData[]>([]);
const loading = ref(true);
const error = ref<Error | null>(null);

const fetchDogImages = async () => {
  try {
    loading.value = true;
    error.value = null;

    const result = await invoke("fetch_dog_images");

    dogData.value = result as DogData[];

  } catch (err) {
    error.value = err as Error;
  } finally {
    loading.value = false;
  }
};

onMounted(fetchDogImages);
</script>

<template>
  <div
    class="h-full w-full flex flex-col justify-center items-center p-12 overflow-auto"
  >
    <Card class="p-8">
      <template #title>
        <div class="flex justify-center items-center font-bold">
          <h2>Jasons Hundebilder</h2>
        </div>
      </template>
      <template #content>
        <div class="flex justify-center items-center">
          <div v-if="loading">
            <Skeleton width="20rem" height="20rem"></Skeleton>
          </div>
          <div v-else-if="error">
            Fehler beim Laden der Bilder: {{ error }}
          </div>
          <div v-else-if="dogData.length === 0">Keine Hundebilder gefunden</div>
          <div v-else>
            <div class="flex flex-col justify-center items-center font-bold">
              <Image
                :src="dogData[0].url"
                alt="Hundebild"
                width="400"
                preview
              />
              <Divider />
                <div class="flex flex-row gap-2">
                  Name: {{ dogData[0].breeds[0].name }}
                </div>
                <div class="flex flex-row gap-2">
                  Rasse: {{ dogData[0].breeds[0].breed_group || 'N/A' }}
                </div>
                <div class="flex flex-row gap-2">
                  Lebenszeit: {{ dogData[0].breeds[0].life_span }}
                </div>
            </div>
          </div>
        </div>
      </template>
      <template #footer>
        <div class="flex justify-evenly">
          <Button label="Neues Bild" class="mt-4" @click="fetchDogImages" fluid>
            <template #icon>
              <RefreshCcw />
            </template>
          </Button>
        </div>
      </template>
    </Card>
  </div>
</template>

