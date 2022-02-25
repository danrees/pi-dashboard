export type DateTime = {
  dateTime: Date;
};

export type Event = {
  id: string;
  summary: string;
  start: DateTime;
};

export type EventList = {
  items: Event[];
};
