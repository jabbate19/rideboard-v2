import type { PopupType } from '@/models'
import { defineStore } from 'pinia'
import { v4 as uuidv4 } from 'uuid'

interface PopupMessage {
  uuid: string
  alertType: PopupType
  text: string
}

export const usePopupStore = defineStore('popup', {
  state: () => ({
    popupMap: {} as Record<string, PopupMessage>
  }),
  getters: {
    popups: (state) => Object.values(state.popupMap)
  },
  actions: {
    addPopup(alertType: PopupType, text: string) {
      const uuid = uuidv4()
      this.popupMap[uuid] = { uuid, alertType, text }
      setTimeout(this.deletePopup, 5000, uuid)
    },
    deletePopup(uuid: string) {
      delete this.popupMap[uuid]
    }
  }
})
