interface FormInputProps {
  label: string;
  type?: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  required?: boolean;
  className?: string;
  autoComplete?: string;
  error?: string;
  disabled?: boolean;
  rows?: number;
}

export default function FormInput({
  label,
  type = "text",
  value,
  onChange,
  placeholder,
  required = false,
  className = "",
  autoComplete,
  error,
  disabled = false,
  rows = 3
}: FormInputProps) {
  const baseClasses = `mt-1 block w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 ${
    error ? 'border-red-300 focus:ring-red-500 focus:border-red-500' : 'border-gray-300'
  } ${disabled ? 'bg-gray-100 cursor-not-allowed' : ''} ${className}`;

  return (
    <div>
      <label className="block text-sm font-medium text-gray-700">{label}</label>
      {type === 'textarea' ? (
        <textarea
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          required={required}
          disabled={disabled}
          rows={rows}
          className={baseClasses}
        />
      ) : (
        <input
          type={type}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          required={required}
          autoComplete={autoComplete}
          disabled={disabled}
          className={baseClasses}
        />
      )}
      {error && (
        <p className="mt-1 text-sm text-red-600">{error}</p>
      )}
    </div>
  );
}
