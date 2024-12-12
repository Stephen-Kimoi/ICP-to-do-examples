import { useState, useEffect, SetStateAction } from 'react';
import { data_persistence_backend } from '../declarations/data_persistence_backend';

interface Note {
  id: bigint;
  title: string;
  content: string;
  created_at: bigint;
}

export default function App() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [loading, setLoading] = useState(false);

  const fetchNotes = async () => {
    const result = await data_persistence_backend.list_notes();
    setNotes(result);
  };

  useEffect(() => {
    fetchNotes();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      await data_persistence_backend.create_note(title, content); 
      // setNotes([...notes, { id: BigInt(1), title, content, created_at: BigInt(1) }]);
      setTitle('');
      setContent('');
      fetchNotes();
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: bigint) => {
    await data_persistence_backend.delete_note(id);
    fetchNotes();
  };

  return (
    <div className="min-h-screen bg-gray-100 py-8">
      <div className="max-w-4xl mx-auto px-4">
        <h1 className="text-3xl font-bold text-gray-900 mb-8">Persistent Notes</h1>
        
        <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow-md p-6 mb-8">
          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Title
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>
          
          <div className="mb-4">
            <label className="block text-gray-700 text-sm font-bold mb-2">
              Content
            </label>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 h-32"
              required
            />
          </div>
          
          <button
            type="submit"
            disabled={loading}
            className="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600 transition-colors disabled:bg-blue-300"
          >
            {loading ? 'Saving...' : 'Save Note'}
          </button>
        </form>

        <div className="grid gap-4 md:grid-cols-2">
          {notes.map((note) => (
            <div key={note.id.toString()} className="bg-white rounded-lg shadow-md p-6">
              <h3 className="text-xl font-semibold mb-2">{note.title}</h3>
              <p className="text-gray-600 mb-4">{note.content}</p>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500">
                  {new Date(Number(note.created_at) / 1000000).toLocaleDateString()}
                </span>
                <button
                  onClick={() => handleDelete(note.id)}
                  className="text-red-500 hover:text-red-600 transition-colors"
                >
                  Delete
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}