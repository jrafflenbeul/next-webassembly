import React from "react";

const Container = ({ children, spacing = false, x = 1, ...props }) => {
  const paddingInPx = 42;
  return (
    <div className="quikkontainer" {...props}>
      {children}
      <style jsx>{`
        .quikkontainer {
          height: 100%;
          width: 100%;
          padding: ${spacing ? 64 * x : 0}px ${paddingInPx}px;
          margin-right: auto;
          margin-left: auto;
        }
        @media (max-width: 575px) {
          .quikkontainer {
            max-width: calc(100vw - ${paddingInPx * 2}px) !important;
          }
        }
        @media (min-width: 576px) {
          .quikkontainer {
            max-width: calc(540px - ${paddingInPx * 2}px) !important;
          }
        }
        @media (min-width: 768px) {
          .quikkontainer {
            max-width: calc(720px - ${paddingInPx * 2}px) !important;
          }
        }
        @media (min-width: 992px) {
          .quikkontainer {
            max-width: calc(960px - ${paddingInPx * 2}px) !important;
          }
        }
        @media (min-width: 1200px) {
          .quikkontainer {
            max-width: calc(1140px - ${paddingInPx * 2}px) !important;
          }
        }
      `}</style>
    </div>
  );
};

export default Container;
