export interface UserData {
  type: string
  id: string
  given_name: string
  family_name: string
  preferred_username: string | undefined
  picture: string | undefined
}

export interface Event {
  id: number
  name: string
  location: string
  start_time: Date
  end_time: Date
}

export interface Car {
  id: number
  driver: string
  riders: string[]
  max_capacity: number
  departure_time: Date
  return_time: Date
  comment: string
}
