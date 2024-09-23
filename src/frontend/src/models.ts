export interface UserData {
  type: string
  id: string
  given_name: string
  family_name: string
  preferred_username: string | undefined
  picture: string | undefined
}

export interface UserStub {
  id: string
  name: string
}

export interface Event {
  id: number
  name: string
  location: string
  startTime: Date
  endTime: Date
  creator: UserStub
  cars?: Car[]
}

export interface Car {
  id: number
  driver: UserStub
  riders: UserStub[]
  maxCapacity: number
  departureTime: Date
  returnTime: Date
  comment: string
}

export enum PopupType {
  Danger = 'bg-danger',
  Warning = 'bg-warning',
  Success = 'bg-success',
  Default = 'bg-primary'
}
