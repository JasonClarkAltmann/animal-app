// { 
//     "breeds": 
//     [ 
//         { 
//             "weight": 
//             { 
//                 "imperial": "90 - 120", 
//                 "metric": "41 - 54" 
//             },
//              "height": 
//              { "imperial": "28 - 34",
//                 "metric": "71 - 86" 
//             }, 
//             "id": 5, 
//             "name": "Akbash Dog", 
//             "bred_for": "Sheep guarding", 
//             "breed_group": "Working", 
//             "life_span": "10 - 12 years", 
//             "temperament": "Loyal, Independent, Intelligent, Brave", 
//             "origin": "", 
//             "reference_image_id": "26pHT3Qk7"
//         } 
//     ], 
//     "id": "GYC8Oeux6", 
//     "url": "https://cdn2.thedogapi.com/images/GYC8Oeux6.jpg", 
//     "width": 800, 
//     "height": 533 
// }

export interface DogData {
  breeds: {
    weight: {
      imperial: string;
      metric: string;
    };
    height: {
      imperial: string;
      metric: string;
    };
    id: number;
    name: string;
    bred_for: string;
    breed_group: string;
    life_span: string;
    temperament: string;
    origin: string;
    reference_image_id: string;
  }[];
  id: string;
  url: string;
  width: number;
  height: number;
}