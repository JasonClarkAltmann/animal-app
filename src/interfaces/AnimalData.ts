export interface AnimalData {
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
