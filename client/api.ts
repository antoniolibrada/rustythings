export type Todo = {
  title: string;
  id: string;
  completed: boolean;
};

export function list(): Promise<Todo[]> {
  return fetch("/todo").then((res) => res.json());
}

export function add({ title }: Pick<Todo, "title">): Promise<Todo> {
  return fetch("/todo", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      title,
    }),
  }).then((res) => res.json());
}

export function update(
  id: string,
  { title, completed }: Pick<Todo, "title" | "completed">
): Promise<Todo> {
  return fetch(`/todo/${id}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      title,
      completed,
    }),
  }).then((res) => res.json());
}
