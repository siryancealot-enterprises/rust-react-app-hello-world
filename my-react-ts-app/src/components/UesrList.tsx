import { useState, useEffect } from "react";

interface User {
  id: number;
  name: string;
  email: string;
  username: string;
}

interface UsersState {
  users: User[] | null;
  loading: boolean;
  error: string | null;
}

const useUsers = (): UsersState => {
  const [state, setState] = useState<UsersState>({
    users: null,
    loading: true,
    error: null,
  });

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        const response = await fetch(
        //   "https://jsonplaceholder.typicode.com/users"
            "/api/users"
        );
        const users = await response.json();
        setState({ users, loading: false, error: null });
      } catch (error) {
        setState({ users: null, loading: false, error: "need to get the error message thing working below" });
        // setState({ users: null, loading: false, error: error.message });
      }
    };

    fetchUsers();
  }, []);

  return state;
};

function UserList() {
  const { users, loading, error } = useUsers();

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>{error}</div>;
  }

  return (
    <ul>
      <li>
         {users?.map((user) => (
          <div key={user.id}>
            <p>{user.name}</p>
            <p>{user.email}</p>
            <p>{user.username}</p>
          </div>
        ))}
      </li>
    </ul>
  );
}

export default UserList;