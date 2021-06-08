import init, { f1, f2, Measurements, f3, f4 } from "./pkg/clothes.js";

init().then(() => {
  console.log(f1("test"));
  console.log(f2());
  const m = new Measurements();
  const mbyf = f3();
  m.ankle = 4;
  console.log(f4(m));
});
