<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { AnimalData } from "../interfaces/AnimalData";
import {
  Bookmark,
  ExternalLink,
  Heart,
  Hourglass,
  RefreshCcw,
  Weight,
} from "lucide-vue-next";
import { Button, Dialog, Image, Skeleton } from "primevue";

const dogData = ref<AnimalData[]>([]);
const likedDogData = ref<AnimalData[]>([]);

const loading = ref(false);
const error = ref<Error | null>(null);

const liked = ref(false);
const displayLikes = ref(false);

const checkLikedStatus = () => {
  if (dogData.value.length > 0) {
    const currentDogUrl = dogData.value[0].url;
    liked.value = likedDogData.value.some((dog) => dog.url === currentDogUrl);
  } else {
    liked.value = false;
  }
};

const fetchDogData = async () => {
  try {
    loading.value = true;
    error.value = null;
    liked.value = false;

    const result = await invoke("fetch_dog_data");

    dogData.value = result as AnimalData[];
    await fetchLikedDogData();
    checkLikedStatus();
  } catch (err) {
    error.value = err as Error;
  } finally {
    loading.value = false;
  }
};

const fetchLikedDogData = async () => {
  try {
    const result = await invoke("get_liked_dog_data");
    likedDogData.value = result as AnimalData[];
    checkLikedStatus();
  } catch (err) {
    console.error("Fehler beim Abrufen der gelikten Hundebilder:", err);
  }
};

const toggleLike = async () => {
  if (dogData.value.length === 0) return;

  const currentDog = dogData.value[0];
  try {
    loading.value = true;
    error.value = null;

    if (liked.value) {
      await invoke("unlike_dog", { dogUrl: currentDog.url });
      liked.value = false;
    } else {
      await invoke("like_dog", { dog: currentDog });
      liked.value = true;
    }
    await fetchLikedDogData();
  } catch (err) {
    error.value = err as Error;
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchDogData();
});
</script>

<template>
  <div class="flex flex-col gap-8 p-4">
    <div class="flex flex-col flex-grow justify-center items-center">
      <div v-if="loading">
        <div class="flex flex-col justify-center items-center gap-4">
          <Skeleton width="20rem" height="2rem"></Skeleton>
          <Skeleton width="24rem" height="24rem"></Skeleton>
          <div class="flex flex-row w-full justify-between">
            <Skeleton width="10rem" height="2rem"></Skeleton>
            <Skeleton width="10rem" height="2rem"></Skeleton>
          </div>
        </div>
      </div>
      <div v-else-if="error">
        Fehler beim Laden der Bilder: {{ error.message || error }}
      </div>
      <div v-else-if="dogData.length === 0">Keine Hundebilder gefunden</div>
      <div
        v-else
        class="flex flex-col gap-8 justify-center items-center font-bold"
      >
        <div class="flex flex-row gap-2 text-3xl">
          <Bookmark :size="35" />
          {{ dogData[0].breeds[0]?.name || "N/A" }}
        </div>

        <Image :src="dogData[0].url" alt="Hundebild" width="400" preview />

        <div class="flex w-full justify-between gap-2">
          <div class="flex flex-row gap-2 text-xl">
            <Weight :size="28" />
            {{ dogData[0].breeds[0].weight.metric + " kg" || "N/A" }}
          </div>
          <div class="flex flex-row gap-2 text-xl">
            <Hourglass :size="28" />
            {{
              dogData[0].breeds[0]?.life_span
                .replace("years", "Jahre")
                .trim() || "N/A"
            }}
          </div>
        </div>
      </div>
    </div>

    <div class="flex flex-row justify-evenly items-center gap-4">
      <Button label=" " @click="toggleLike" class="w-45" fluid rounded>
        <template #icon>
          <Heart v-if="!liked" />
          <Heart v-else fill="#ff6467" />
        </template>
      </Button>

      <Button
        label="Neues Bild"
        @click="fetchDogData"
        class="w-45"
        fluid
        rounded
      >
        <template #icon>
          <RefreshCcw />
        </template>
      </Button>
    </div>

    <Button
      v-if="likedDogData.length > 0"
      label="Gelikte Hunde"
      fluid
      rounded
      @click="displayLikes = true"
    >
      <template #icon>
        <ExternalLink />
      </template>
    </Button>

    <Dialog v-model:visible="displayLikes" modal>
      <template #header>
        <Heart />
        <div class="font-bold">Gelikte Hunde</div>
      </template>
      <div class="grid grid-cols-3 gap-4">
        <div
          v-for="dog in likedDogData"
          :key="dog.url"
          class="flex flex-col items-center"
        >
          <div
            class="w-36 h-36 flex items-center justify-center overflow-hidden"
          >
            <Image
              :src="dog.url"
              :alt="dog.breeds[0]?.name || 'Hundebild'"
              class="max-w-full max-h-full object-contain"
              preview
            />
          </div>
          <p class="text-sm mt-1 text-center">
            {{ dog.breeds[0]?.name || "Unbekannt" }}
          </p>
        </div>
      </div>
    </Dialog>
  </div>
</template>
