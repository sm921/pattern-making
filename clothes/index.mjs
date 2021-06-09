import init, { Base, Measurements } from "./pkg/clothes.js";

init().then(() => {
  const m = new Measurements();
  m.waist = 60.0;
  m.hps_to_waist = 57.0;
  m.nape_to_waist = 57.0;
  m.armscye_depth = 21.0;
  m.neck_size = 27.0;
  m.shoulder = 13.0;
  m.x_front = 27.0;
  const base = new Base(m, 14);
  base.draw(900, 900);
  console.log(base);
});
