interface CardProps {
  children: React.ReactNode;
  className?: string;
  hover?: boolean;
  maxHeight?: string;
}

export default function Card({ children, className = "", hover = false, maxHeight = "400px" }: CardProps) {
  const hoverClass = hover ? "hover:shadow-lg transition-shadow duration-200" : "";
  
  return (
    <div 
      className={`bg-white shadow rounded-lg ${hoverClass} ${className}`}
      style={{ 
        maxHeight: maxHeight,
        overflowY: 'auto'
      }}
    >
      {children}
    </div>
  );
}
