interface CardProps {
  children: React.ReactNode;
  className?: string;
  hover?: boolean;
}

export default function Card({ children, className = "", hover = false }: CardProps) {
  const hoverClass = hover ? "hover:shadow-lg transition-shadow duration-200" : "";
  
  return (
    <div className={`bg-white shadow rounded-lg ${hoverClass} ${className}`}>
      {children}
    </div>
  );
}
