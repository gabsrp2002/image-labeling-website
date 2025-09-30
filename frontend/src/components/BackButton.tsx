interface BackButtonProps {
  onClick: () => void;
  children?: React.ReactNode;
  className?: string;
}

export default function BackButton({ onClick, children = "Back", className = "" }: BackButtonProps) {
  return (
    <button
      onClick={onClick}
      className={`text-blue-600 hover:text-blue-800 flex items-center text-sm sm:text-base ${className}`}
    >
      <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
      </svg>
      {children}
    </button>
  );
}
