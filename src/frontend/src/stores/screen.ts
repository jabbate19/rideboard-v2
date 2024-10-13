import { defineStore } from 'pinia';

export const useScreenStore = defineStore('screen', {
  state: () => ({
    width: window.innerWidth
  }),
  getters: {
    mobile: (state) => state.width < 768
  }
});

const updateWidth = () => {
  const screenStore = useScreenStore();
  screenStore.width = window.innerWidth;
};

window.addEventListener('resize', updateWidth);
