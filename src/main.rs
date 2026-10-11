use eframe::egui::{self, *};
use std::collections::HashMap;

const SECTIONS: &[(&str, &str, &str, &str)] = &[
 ("01","フェードイン","Fade-in","淡入"),
 ("02","スライドイン","Slide-in","滑入"),
 ("03","スタッガー","Stagger animation","交錯／依序出現"),
 ("04","テキストリビール","Text reveal (char by char)","文字逐字顯現"),
 ("05","マスクリビール","Mask / Clip-path reveal","遮罩揭露"),
 ("06","ブラーイン","Blur-in","模糊淡入"),
 ("07","カウントアップ","Count-up numbers","數字跳動"),
 ("08","プログレスバー","Progress bar fill","進度條填充"),
 ("09","スクロール連動","Scroll-linked animation","滾動連動"),
 ("10","パララックス","Parallax","視差滾動"),
 ("11","スクロールズーム","Scale on scroll","滾動縮放"),
 ("12","画像シーケンス","Image sequence (scrub)","圖片序列"),
 ("13","SVGラインドロー","SVG line draw","SVG 描線"),
 ("14","テキストフィル","Text fill on scroll","滾動文字填色"),
 ("15","スティッキー","Sticky / Pinning","固定／釘選"),
 ("16","スクロールスナップ","Scroll snap","滾動吸附"),
 ("17","マーキー","Marquee / Infinite loop","跑馬燈／無限循環"),
 ("18","ズームイン","Zoom-in","放大進場"),
 ("19","フリップイン","Flip / Rotate-in","翻轉／旋轉進場"),
 ("20","バウンス","Bounce","彈跳"),
 ("21","スプリング","Elastic / Spring","彈性／彈簧"),
 ("22","タイプライター","Typewriter","打字機效果"),
 ("23","テキストスクランブル","Text scramble / Shuffle","文字亂碼解碼"),
 ("24","グラデーションシマー","Gradient text shimmer","漸層文字流光"),
 ("25","スプリットテキストウェーブ","Split-text wave","拆字波浪"),
 ("26","3Dチルト","Hover lift / 3D tilt","懸停浮起／3D 傾斜"),
 ("27","マグネティックボタン","Magnetic button","磁吸按鈕"),
 ("28","リップル","Ripple","水波紋"),
 ("29","モーフィング","Shape morphing","形狀變形"),
 ("30","横スクロール","Horizontal scroll section","橫向捲動區塊"),
 ("31","カーテン／ワイプ","Curtain / Wipe transition","布幕／擦除轉場"),
 ("32","スケルトンシマー","Skeleton shimmer","骨架屏流光"),
 ("33","ローディングスピナー","Loading spinners","載入轉圈"),
 ("34","パーティクルバースト","Particle burst","粒子爆發"),
 ("35","グリッチ","Glitch","故障雜訊"),
 ("36","パルス","Pulse","脈動"),
 ("37","シェイク","Shake","搖晃"),
 ("38","アコーディオン","Accordion expand","手風琴展開"),
 ("39","カードフリップ","Card flip","卡片翻面"),
 ("40","プログレスリング","Progress ring","環形進度"),
 ("41","双方向マーキー","Bi-directional marquee","雙向跑馬燈"),
 ("42","カーソルフォロワー","Cursor follower","游標跟隨"),
 ("43","スクロール速度スキュー","Scroll velocity skew","捲動速度傾斜"),
 ("44","ホバー画像リビール","Reveal-on-hover image","懸停顯示圖片"),
 ("45","スタッガーグリッド","Staggered grid","交錯網格"),
 ("46","クロスディゾルブ","Cross dissolve","交叉溶接"),
 ("47","フェードアウト（黒）","Fade to black","淡出至黑"),
 ("48","ウィップパン","Whip pan","甩鏡轉場"),
 ("49","ズームトランジション","Zoom transition","縮放轉場"),
 ("50","ケン・バーンズ","Ken Burns effect","肯·伯恩斯效果"),
 ("51","スピードランプ","Slow motion / Speed ramp","慢動作／變速"),
 ("52","フリーズフレーム","Freeze frame","定格畫面"),
 ("53","分割画面","Split screen","分割畫面"),
 ("54","ピクチャーインピクチャー","Picture-in-picture","子母畫面"),
 ("55","ライトリーク","Light leak","漏光"),
 ("56","フィルムグレイン","Film grain","底片顆粒"),
 ("57","ビネット","Vignette","暗角"),
 ("58","色収差","Chromatic aberration","色差"),
 ("59","RGBスプリット","RGB split","RGB 分離"),
 ("60","レターボックス","Letterbox","電影黑邊"),
 ("61","カラーグレーディング","Color grading / LUT","調色／LUT"),
 ("62","ロワーサード","Lower third","下方字幕條"),
 ("63","キネティックタイポグラフィ","Kinetic typography","動態文字排版"),
 ("64","マッチカット","Match cut","匹配剪輯"),
 ("65","集中線","Speed lines","集中線／速度線"),
 ("66","インパクトフレーム","Impact frame","衝擊幀"),
 ("67","スメアフレーム","Smear frame","拖影幀"),
 ("68","画面揺れ","Screen shake","畫面震動"),
 ("69","桜吹雪","Sakura particles","櫻花飄落"),
 ("70","ちびポップ","Chibi pop","Q版彈出"),
 ("71","汗マーク","Sweat drop","汗滴"),
 ("72","怒りマーク","Anger mark","怒氣符號"),
 ("73","キラキラ","Kira sparkle","閃亮光芒"),
 ("74","ホワイトアウト","Flash / White-out","閃白"),
 ("75","漫画コマ","Manga panel frame","漫畫分格"),
 ("76","ダッチアングル","Dutch angle","傾斜鏡頭"),
 ("77","背景パン","Background pan","背景平移"),
 ("78","アイキャッチ","Eyecatch","過場卡"),
 ("79","アイリスワイプ","Iris wipe","圓形擦除"),
 ("80","スターワイプ","Star wipe","星形擦除"),
 ("81","クロックワイプ","Clock wipe","時鐘擦除"),
 ("82","ラックフォーカス","Rack focus","移焦"),
 ("83","ドリーズーム","Dolly zoom (Vertigo)","滑動變焦／眩暈效果"),
 ("84","ボケ","Bokeh","散景"),
 ("85","レンズフレア","Lens flare","鏡頭光暈"),
 ("86","雨オーバーレイ","Rain overlay","下雨疊加"),
 ("87","CRT／VHS","CRT / VHS","CRT／錄影帶效果"),
 ("88","フィルムグリッチ","Film glitch / burn","膠片故障"),
 ("89","エンドロール","Title credits roll","片尾字幕捲動"),
 ("90","カウントダウンリーダー","Countdown leader","倒數片頭"),
 ("91","古いフィルムのちらつき","Old film flicker / scratches","老電影閃爍刮痕"),
];

fn ease(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); 1.0 - (1.0 - t).powi(3) }

struct App { start: HashMap<usize, f64>, snap: bool, cur: usize }

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 760.0]).with_title("Web Scroll Animation Showcase"), ..Default::default() };
    eframe::run_native("anim", opts, Box::new(|cc| {
        let mut f = FontDefinitions::default();
        f.font_data.insert("cjk".into(), FontData::from_static(include_bytes!("../assets/NotoSansCJK-Regular.ttc")));
        for fam in [FontFamily::Proportional, FontFamily::Monospace] { f.families.get_mut(&fam).unwrap().push("cjk".into()); }
        cc.egui_ctx.set_fonts(f);
        cc.egui_ctx.set_visuals(Visuals::dark());
        Ok(Box::new(App { start: HashMap::new(), snap: false, cur: 0 }))
    }))
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        let mut progress_total = 0.0;
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("トレンド Web スクロールアニメーション / 網頁滾動動畫圖鑑");
                if ui.button("⟲ Replay 重播").clicked() { self.start.clear(); }
                ui.checkbox(&mut self.snap, "Scroll snap 吸附");
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            let view = ui.max_rect(); let ptr = ctx.input(|i| i.pointer.hover_pos()); let vel = ctx.input(|i| i.smooth_scroll_delta.y);
            let out = ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                let w = ui.available_width();
                ui.add_space(40.0);
                ui.label(RichText::new("↓ 往下滾動，每個區塊進入畫面時播放動畫").size(22.0));
                ui.add_space(view.height() * 0.5);
                for (i, s) in SECTIONS.iter().enumerate() {
                    if let Some(hd) = match i { 0 => Some("🌐 網頁滾動動畫 Web scroll"), 17 => Some("✨ 網頁互動／微動畫 UI micro-interactions"), 45 => Some("🎬 影片剪輯效果 Video editing"), 64 => Some("🌸 動畫／漫畫效果 Anime"), 78 => Some("🎞 電影轉場與鏡頭 Film"), _ => None } { ui.add_space(30.0); ui.label(RichText::new(hd).size(30.0).strong().color(Color32::from_rgb(120, 230, 160))); ui.separator(); }
                    let h = 300.0;
                    let (rect, _) = ui.allocate_exact_size(vec2(w, h), Sense::hover());
                    let p = ui.painter_at(rect);
                    p.rect_filled(rect.shrink(8.0), 12.0, Color32::from_gray(28));
                    // scroll progress: 0 when top at viewport bottom, 1 when bottom at viewport top
                    let sp = ((view.bottom() - rect.top()) / (view.height() + h)).clamp(0.0, 1.0);
                    let visible = rect.top() < view.bottom() - 60.0 && rect.bottom() > view.top();
                    if rect.top() <= view.top() + 120.0 { self.cur = i; }
                    if visible { self.start.entry(i).or_insert(now); }
                    let t = self.start.get(&i).map(|s0| (now - s0) as f32).unwrap_or(0.0);
                    p.text(rect.left_top() + vec2(24.0, 18.0), Align2::LEFT_TOP, format!("{}  {}  /  {}  /  {}", s.0, s.1, s.2, s.3), FontId::proportional(22.0), Color32::from_rgb(255, 200, 90));
                    let area = Rect::from_min_max(rect.min + vec2(24.0, 60.0), rect.max - vec2(24.0, 20.0));
                    draw(i, &p, area, t, sp, now as f32, ptr, vel);
                }
                ui.add_space(view.height());
            });
            let off = out.state.offset.y;
            let max = (out.content_size.y - out.inner_rect.height()).max(1.0);
            progress_total = off / max;
            // 09: global progress bar at top of viewport
            let p = ui.painter();
            p.rect_filled(Rect::from_min_size(view.min, vec2(view.width() * progress_total, 5.0)), 0.0, Color32::from_rgb(90, 200, 255));
            // 15 sticky header: shows current section pinned
            let idx = self.cur;
            p.text(view.right_top() + vec2(-16.0, 12.0), Align2::RIGHT_TOP, format!("📌 Sticky: {} {}", SECTIONS[idx].0, SECTIONS[idx].3), FontId::proportional(16.0), Color32::LIGHT_GREEN);
            if self.snap && !ctx.input(|i| i.pointer.any_down()) && ctx.input(|i| i.smooth_scroll_delta.y == 0.0) {
                let base = view.height() * 0.5 + 70.0;
                let target = base + ((off - base) / 300.0).round() * 300.0;
                if (target - off).abs() > 1.0 && off > base - 150.0 {
                    let mut st = out.state; st.offset.y += (target - off) * 0.2; st.store(ctx, out.id);
                }
            }
        });
        ctx.request_repaint();
    }
}

fn draw(i: usize, p: &Painter, a: Rect, t: f32, sp: f32, now: f32, ptr: Option<Pos2>, vel: f32) {
    let accent = Color32::from_rgb(90, 200, 255);
    let c = a.center();
    let card = |p: &Painter, r: Rect, alpha: f32| p.rect_filled(r, 10.0, accent.gamma_multiply(alpha));
    match i {
        0 => { let e = ease(t / 1.2); card(p, Rect::from_center_size(c, vec2(320.0, 140.0)), e); p.text(c, Align2::CENTER_CENTER, "opacity 0 → 1", FontId::proportional(24.0), Color32::BLACK.gamma_multiply(e)); }
        1 => { let e = ease(t / 1.0); card(p, Rect::from_center_size(c + vec2(-300.0 * (1.0 - e), 0.0), vec2(320.0, 140.0)), e); }
        2 => for k in 0..6 { let e = ease((t - k as f32 * 0.15) / 0.7); let x = a.left() + 20.0 + k as f32 * 150.0; card(p, Rect::from_min_size(pos2(x, a.top() + 40.0 + 40.0 * (1.0 - e)), vec2(130.0, 130.0)), e); },
        3 => { let s: Vec<char> = "一個字一個字地出現 Text reveal!".chars().collect(); let n = ((t * 12.0) as usize).min(s.len()); let txt: String = s[..n].iter().collect(); p.text(c, Align2::CENTER_CENTER, txt, FontId::proportional(40.0), Color32::WHITE); }
        4 => { let e = ease(t / 1.4); let r = Rect::from_center_size(c, vec2(600.0, 160.0)); let clip = Rect::from_min_max(r.min, pos2(r.left() + r.width() * e, r.bottom())); let pp = p.with_clip_rect(clip); pp.rect_filled(r, 10.0, Color32::from_rgb(250, 120, 160)); pp.text(r.center(), Align2::CENTER_CENTER, "clip-path: inset()", FontId::proportional(36.0), Color32::BLACK); }
        5 => { let e = ease(t / 1.5); let blur = (1.0 - e) * 14.0; for k in 0..8 { let o = (k as f32 - 3.5) / 3.5 * blur; p.text(c + vec2(o, 0.0), Align2::CENTER_CENTER, "Blur-in 模糊淡入", FontId::proportional(48.0), Color32::WHITE.gamma_multiply(e / if blur > 0.5 { 4.0 } else { 1.0 })); } }
        6 => { let e = ease(t / 2.0); for (k, (v, u)) in [(12800.0, "users"), (98.0, "%"), (365.0, "days")].iter().enumerate() { p.text(pos2(a.left() + 150.0 + k as f32 * 300.0, c.y), Align2::CENTER_CENTER, format!("{:.0} {}", v * e, u), FontId::proportional(44.0), accent); } }
        7 => for (k, v) in [0.9, 0.7, 0.55].iter().enumerate() { let e = ease((t - k as f32 * 0.2) / 1.5) * v; let y = a.top() + 30.0 + k as f32 * 55.0; p.rect_filled(Rect::from_min_size(pos2(a.left(), y), vec2(a.width(), 26.0)), 13.0, Color32::from_gray(50)); p.rect_filled(Rect::from_min_size(pos2(a.left(), y), vec2(a.width() * e, 26.0)), 13.0, accent); p.text(pos2(a.right() - 8.0, y + 13.0), Align2::RIGHT_CENTER, format!("{:.0}%", e * 100.0), FontId::proportional(16.0), Color32::WHITE); },
        8 => { p.rect_filled(Rect::from_min_size(pos2(a.left(), c.y), vec2(a.width(), 12.0)), 6.0, Color32::from_gray(50)); p.rect_filled(Rect::from_min_size(pos2(a.left(), c.y), vec2(a.width() * sp, 12.0)), 6.0, accent); p.text(pos2(a.left(), c.y - 40.0), Align2::LEFT_CENTER, format!("跟著捲動位置變化 progress = {:.2}（頂部藍條為全頁進度）", sp), FontId::proportional(20.0), Color32::WHITE); }
        9 => { let pp = p.with_clip_rect(a); for (k, &(_r, depth)) in [(0.2f32, 60.0f32), (0.6, 160.0), (1.2, 300.0)].iter().enumerate() { let y = a.bottom() - (sp - 0.5) * depth - 60.0 - k as f32 * 30.0; for j in 0..5 { pp.circle_filled(pos2(a.left() + 80.0 + j as f32 * 200.0 + k as f32 * 60.0, y), 14.0 + k as f32 * 10.0, accent.gamma_multiply(0.3 + 0.3 * k as f32)); } } p.text(a.left_top(), Align2::LEFT_TOP, "遠景慢、近景快", FontId::proportional(18.0), Color32::GRAY); }
        10 => { let s = 0.4 + 1.0 * sp; card(p, Rect::from_center_size(c, vec2(260.0 * s, 110.0 * s)).intersect(a), 1.0); p.text(c, Align2::CENTER_CENTER, format!("scale {:.2}", s), FontId::proportional(24.0), Color32::BLACK); }
        11 => { let f = (sp * 23.99) as usize; let ang = f as f32 / 24.0 * std::f32::consts::TAU; p.circle_stroke(c, 80.0, Stroke::new(3.0, Color32::GRAY)); p.line_segment([c, c + vec2(ang.cos(), ang.sin()) * 80.0], Stroke::new(8.0, accent)); p.text(pos2(c.x + 140.0, c.y), Align2::LEFT_CENTER, format!("frame {:02}/24", f + 1), FontId::proportional(24.0), Color32::WHITE); }
        12 => { let e = ease(t / 2.5); let n = 200; let pts: Vec<Pos2> = (0..=((n as f32 * e) as usize)).map(|k| { let x = k as f32 / n as f32; pos2(a.left() + 40.0 + x * (a.width() - 80.0), c.y + (x * 12.0).sin() * 60.0 * (1.0 - x * 0.5)) }).collect(); p.add(Shape::line(pts, Stroke::new(4.0, accent))); }
        13 => { let txt = "Scroll to fill 捲動填色"; let font = FontId::proportional(56.0); let g = p.layout_no_wrap(txt.into(), font.clone(), Color32::from_gray(70)); let r = Align2::CENTER_CENTER.anchor_size(c, g.size()); p.galley(r.min, g, Color32::WHITE); let fill = ((sp - 0.25) * 2.0).clamp(0.0, 1.0); let pp = p.with_clip_rect(Rect::from_min_max(r.min, pos2(r.left() + r.width() * fill, r.bottom()))); pp.text(c, Align2::CENTER_CENTER, txt, font, Color32::WHITE); }
        14 => { p.text(c, Align2::CENTER_CENTER, "右上角 📌 標籤固定在畫面上（position: sticky）", FontId::proportional(24.0), Color32::LIGHT_GREEN); }
        15 => { p.text(c, Align2::CENTER_CENTER, "勾選上方「Scroll snap」：停止捲動後自動對齊區塊", FontId::proportional(24.0), Color32::WHITE); }
        16 => { let pp = p.with_clip_rect(a); let txt = "MARQUEE ✦ 跑馬燈 ✦ INFINITE LOOP ✦ マーキー ✦ "; let g = pp.layout_no_wrap(txt.into(), FontId::proportional(40.0), accent); let w = g.size().x; let off = (now * 120.0) % w; for k in 0..4 { pp.galley(pos2(a.left() - off + k as f32 * w, c.y - 20.0), g.clone(), accent); } }
        45.. => draw3(i - 45, p, a, t, sp, now),
        _ => draw2(i - 17, p, a, t, sp, now, ptr, vel),
    }
}

fn rot(c: Pos2, v: Vec2, a: f32) -> Pos2 { c + vec2(v.x * a.cos() - v.y * a.sin(), v.x * a.sin() + v.y * a.cos()) }
fn quad(p: &Painter, c: Pos2, w: f32, h: f32, ang: f32, sx: f32, col: Color32) {
    let pts = [vec2(-w, -h), vec2(w, -h), vec2(w, h), vec2(-w, h)].iter().map(|v| rot(c, vec2(v.x * sx, v.y), ang)).collect();
    p.add(Shape::convex_polygon(pts, col, Stroke::NONE));
}
fn hsv(h: f32) -> Color32 { ecolor::Hsva::new(h.rem_euclid(1.0), 0.7, 1.0, 1.0).into() }

#[allow(clippy::too_many_arguments)]
fn draw2(k: usize, p: &Painter, a: Rect, t: f32, sp: f32, now: f32, ptr: Option<Pos2>, vel: f32) {
    use std::f32::consts::{PI, TAU};
    let ac = Color32::from_rgb(90, 200, 255);
    let c = a.center();
    let f = |s: f32| FontId::proportional(s);
    let pp = p.with_clip_rect(a);
    let hover = ptr.filter(|q| a.contains(*q));
    let loop_t = t % 3.0;
    match k {
        0 => { let e = ease(t / 0.8); quad(p, c, 160.0 * (0.3 + 0.7 * e), 70.0 * (0.3 + 0.7 * e), 0.0, 1.0, ac.gamma_multiply(e)); }
        1 => { let e = ease(t / 1.0); quad(p, c, 160.0, 70.0, (1.0 - e) * -PI / 2.0, 1.0, ac.gamma_multiply(e)); quad(p, c + vec2(300.0, 0.0), 80.0, 70.0, 0.0, (e * PI / 2.0).sin().max(0.02), Color32::from_rgb(250, 120, 160)); }
        2 => { let x = loop_t / 1.5; let y = if x < 1.0 { let b = 1.0 - x; (b * PI * 3.0).sin().abs() * b * b } else { 0.0 }; p.circle_filled(pos2(c.x, a.bottom() - 30.0 - y * 150.0), 30.0, ac); }
        3 => { let x = loop_t; let s = 1.0 + (-(x * 4.0)).exp() * (x * 18.0).sin() * 0.6 - (-(x*4.0)).exp()*0.0; quad(p, c, 90.0 * s, 70.0 * (2.0 - s), 0.0, 1.0, ac); }
        4 => { let s = "> Hello, typewriter_ 打字機"; let ch: Vec<char> = s.chars().collect(); let n = ((t * 10.0) as usize).min(ch.len()); let mut out: String = ch[..n].iter().collect(); if (now * 2.0) as i32 % 2 == 0 { out.push('▌'); } p.text(pos2(a.left() + 40.0, c.y), Align2::LEFT_CENTER, out, FontId::monospace(36.0), Color32::LIGHT_GREEN); }
        5 => { let tgt: Vec<char> = "DECODING 解碼完成".chars().collect(); let pool: Vec<char> = "!<>-_\\/[]{}=+*^?#ABXZ01".chars().collect(); let out: String = tgt.iter().enumerate().map(|(j, &ch)| if t * 8.0 > j as f32 + 4.0 { ch } else { pool[((now * 30.0) as usize + j * 7) % pool.len()] }).collect(); p.text(c, Align2::CENTER_CENTER, out, FontId::monospace(44.0), Color32::WHITE); }
        6 => { let txt = "Gradient Shimmer 流光文字"; let g = p.layout_no_wrap(txt.into(), f(52.0), Color32::WHITE); let r = Align2::CENTER_CENTER.anchor_size(c, g.size()); let n = 40; for j in 0..n { let x0 = r.left() + r.width() * j as f32 / n as f32; let cl = Rect::from_min_max(pos2(x0, r.top()), pos2(x0 + r.width() / n as f32 + 1.0, r.bottom())); p.with_clip_rect(cl).galley(r.min, g.clone(), hsv(j as f32 / n as f32 * 0.5 - now * 0.4)); } }
        7 => { let txt: Vec<char> = "WAVE TEXT 波浪文字".chars().collect(); let mut x = a.left() + 120.0; for (j, ch) in txt.iter().enumerate() { let y = (now * 5.0 - j as f32 * 0.5).sin() * 20.0; let g = p.layout_no_wrap(ch.to_string(), f(48.0), Color32::WHITE); let w = g.size().x; p.galley(pos2(x, c.y - 24.0 + y), g, hsv(j as f32 * 0.05)); x += w; } }
        8 => { let r = Rect::from_center_size(c, vec2(320.0, 160.0)); let (dx, dy) = hover.map(|q| ((q.x - c.x) / 300.0, (q.y - c.y) / 100.0)).unwrap_or((0.0, 0.0)); let lift = if hover.is_some() { 10.0 } else { 0.0 }; p.rect_filled(r.translate(vec2(0.0, 12.0)), 12.0, Color32::from_black_alpha(120)); let pts: Vec<Pos2> = [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom()].iter().map(|q| { let v = *q - c; c + vec2(v.x * (1.0 + dy * v.y.signum() * 0.08), v.y * (1.0 - dx * v.x.signum() * 0.08)) - vec2(0.0, lift) }).collect(); p.add(Shape::convex_polygon(pts, ac, Stroke::NONE)); p.text(c - vec2(0.0, lift), Align2::CENTER_CENTER, "滑鼠移上來試試", f(22.0), Color32::BLACK); }
        9 => { let off = hover.map(|q| { let d = q - c; if d.length() < 220.0 { d * 0.35 } else { Vec2::ZERO } }).unwrap_or(Vec2::ZERO); p.rect_filled(Rect::from_center_size(c + off, vec2(200.0, 64.0)), 32.0, ac); p.text(c + off, Align2::CENTER_CENTER, "Magnetic 磁吸", f(22.0), Color32::BLACK); }
        10 => { p.rect_filled(Rect::from_center_size(c, vec2(360.0, 120.0)), 12.0, Color32::from_gray(60)); for j in 0..3 { let x = (loop_t - j as f32 * 0.5).max(0.0); pp.circle_stroke(c, x * 120.0, Stroke::new(3.0, ac.gamma_multiply((1.0 - x / 2.0).max(0.0)))); } }
        11 => { let m = ((now * 1.2).sin() + 1.0) / 2.0; let n = 64; let pts: Vec<Pos2> = (0..n).map(|j| { let ang = j as f32 / n as f32 * TAU; let circ = vec2(ang.cos(), ang.sin()) * 90.0; let sq = circ / circ.x.abs().max(circ.y.abs()) * 80.0; let star = circ * (1.0 + 0.0); c + sq * (1.0 - m) + star * m }).collect(); p.add(Shape::convex_polygon(pts, hsv(m * 0.3), Stroke::NONE)); }
        12 => { let x = -(sp - 0.2).clamp(0.0, 0.6) / 0.6 * 1400.0; for j in 0..8 { let r = Rect::from_min_size(pos2(a.left() + x + j as f32 * 260.0, a.top() + 20.0), vec2(240.0, 170.0)); pp.rect_filled(r, 10.0, hsv(j as f32 * 0.1)); pp.text(r.center(), Align2::CENTER_CENTER, format!("Panel {}", j + 1), f(26.0), Color32::BLACK); } }
        13 => { let e = ease(t / 1.2); pp.text(c, Align2::CENTER_CENTER, "Curtain 布幕拉開", f(48.0), Color32::WHITE); let hw = a.width() / 2.0 * (1.0 - e); pp.rect_filled(Rect::from_min_size(a.min, vec2(hw, a.height())), 0.0, Color32::from_rgb(160, 30, 50)); pp.rect_filled(Rect::from_min_max(pos2(a.right() - hw, a.top()), a.max), 0.0, Color32::from_rgb(160, 30, 50)); }
        14 => { for j in 0..3 { let r = Rect::from_min_size(pos2(a.left() + 120.0, a.top() + 20.0 + j as f32 * 50.0), vec2(a.width() - 240.0 - j as f32 * 120.0, 30.0)); p.rect_filled(r, 8.0, Color32::from_gray(55)); let sx = r.left() + ((now * 0.8) % 1.5) * r.width() - 100.0; p.with_clip_rect(r).rect_filled(Rect::from_min_size(pos2(sx, r.top()), vec2(100.0, 30.0)), 0.0, Color32::from_gray(85)); } p.circle_filled(pos2(a.left() + 60.0, a.top() + 45.0), 30.0, Color32::from_gray(55)); }
        15 => { let x0 = a.left() + 120.0; let n = 64; let arc: Vec<Pos2> = (0..n).map(|j| { let ang = now * 5.0 + j as f32 / n as f32 * 4.5; pos2(x0, c.y) + vec2(ang.cos(), ang.sin()) * 40.0 }).collect(); p.add(Shape::line(arc, Stroke::new(6.0, ac))); for j in 0..3 { let s = ((now * 5.0 - j as f32).sin() + 1.0) / 2.0; p.circle_filled(pos2(x0 + 200.0 + j as f32 * 40.0, c.y), 8.0 + s * 6.0, ac); } for j in 0..8 { let ang = j as f32 / 8.0 * TAU; let al = ((j as f32 - now * 10.0).rem_euclid(8.0)) / 8.0; p.line_segment([pos2(x0 + 480.0, c.y) + vec2(ang.cos(), ang.sin()) * 20.0, pos2(x0 + 480.0, c.y) + vec2(ang.cos(), ang.sin()) * 40.0], Stroke::new(5.0, Color32::WHITE.gamma_multiply(al))); } }
        16 => { let x = loop_t / 1.5; for j in 0..40 { let ang = j as f32 * 2.4; let sp2 = 80.0 + (j * 37 % 100) as f32 * 1.5; let d = vec2(ang.cos(), ang.sin()) * sp2 * x + vec2(0.0, 120.0 * x * x); pp.circle_filled(c + d, 5.0 * (1.0 - x).max(0.0), hsv(j as f32 * 0.03)); } }
        17 => { let txt = "GLITCH 故障"; let gl = ((now * 7.0).sin() * (now * 13.0).sin()) > 0.6; let o = if gl { 6.0 } else { 0.0 }; p.text(c + vec2(-o, 0.0), Align2::CENTER_CENTER, txt, f(64.0), Color32::from_rgba_unmultiplied(255, 0, 80, 180)); p.text(c + vec2(o, 0.0), Align2::CENTER_CENTER, txt, f(64.0), Color32::from_rgba_unmultiplied(0, 255, 255, 180)); p.text(c, Align2::CENTER_CENTER, txt, f(64.0), Color32::WHITE); if gl { pp.rect_filled(Rect::from_min_size(pos2(a.left(), c.y + ((now * 50.0) % 60.0) - 30.0), vec2(a.width(), 6.0)), 0.0, Color32::from_black_alpha(200)); } }
        18 => { let s = ((now * 4.0).sin() + 1.0) / 2.0; p.circle_filled(c, 40.0 + 30.0 * s, ac.gamma_multiply(0.3 * (1.0 - s))); p.circle_filled(c, 40.0, ac); }
        19 => { let x = (now % 2.0).min(0.6); let dx = (x * 40.0).sin() * 15.0 * (1.0 - x / 0.6); p.rect_filled(Rect::from_center_size(c + vec2(dx, 0.0), vec2(320.0, 56.0)), 8.0, Color32::from_rgb(200, 60, 60)); p.text(c + vec2(dx, 0.0), Align2::CENTER_CENTER, "密碼錯誤 Wrong password", f(20.0), Color32::WHITE); }
        20 => { for j in 0..3 { let open = ease((t - 0.6 - j as f32 * 0.6) / 0.4) * if j == 1 { 1.0 } else { 0.0 }; let y = a.top() + j as f32 * 50.0 + if j == 2 { open * 0.0 + ease((t - 1.2) / 0.4) * 60.0 } else { 0.0 }; p.rect_filled(Rect::from_min_size(pos2(a.left() + 100.0, y), vec2(a.width() - 200.0, 42.0)), 6.0, Color32::from_gray(60)); p.text(pos2(a.left() + 120.0, y + 21.0), Align2::LEFT_CENTER, format!("{} 問題 Q{}", if j == 1 && t > 1.2 { "▼" } else { "▶" }, j + 1), f(18.0), Color32::WHITE); if j == 1 { let h = ease((t - 1.2) / 0.4) * 60.0; pp.rect_filled(Rect::from_min_size(pos2(a.left() + 100.0, y + 44.0), vec2(a.width() - 200.0, h)), 6.0, Color32::from_gray(40)); let _ = open; } } }
        21 => { let ang = ease(((t - 0.5) / 1.0).clamp(0.0, 1.0)) * PI; let sx = ang.cos(); let front = sx > 0.0; quad(p, c, 110.0, 80.0, 0.0, sx.abs().max(0.02), if front { ac } else { Color32::from_rgb(250, 180, 80) }); p.text(c, Align2::CENTER_CENTER, if front { "FRONT 正面" } else { "BACK 背面" }, f(20.0), Color32::BLACK); }
        22 => { let e = ease(t / 2.0) * 0.75; p.circle_stroke(c, 70.0, Stroke::new(12.0, Color32::from_gray(55))); let n = 80; let pts: Vec<Pos2> = (0..=n).map(|j| { let ang = -PI / 2.0 + TAU * e * j as f32 / n as f32; c + vec2(ang.cos(), ang.sin()) * 70.0 }).collect(); p.add(Shape::line(pts, Stroke::new(12.0, ac))); p.text(c, Align2::CENTER_CENTER, format!("{:.0}%", e * 100.0), f(30.0), Color32::WHITE); }
        23 => { for (row, dir) in [(0.0, 1.0), (1.0, -1.0)] { let txt = if dir > 0.0 { "← LEFT 向左 ✦ " } else { "RIGHT 向右 → ✦ " }; let g = pp.layout_no_wrap(txt.into(), f(36.0), ac); let w = g.size().x; let off = (now * 100.0 * dir).rem_euclid(w); for j in -1..8 { pp.galley(pos2(a.left() - off + j as f32 * w, a.top() + 30.0 + row * 70.0), g.clone(), if dir > 0.0 { ac } else { Color32::from_rgb(250, 120, 160) }); } } }
        24 => { if let Some(q) = hover { let lag = q - vec2(((now * 3.0).sin()) * 0.0, 0.0); p.circle_stroke(lag, 24.0, Stroke::new(2.0, ac)); p.circle_filled(q, 5.0, Color32::WHITE); } else { p.text(c, Align2::CENTER_CENTER, "把滑鼠移進來，圓圈會跟著游標", f(22.0), Color32::GRAY); } }
        25 => { let skew = (vel * 0.02).clamp(-0.5, 0.5); let r = Rect::from_center_size(c, vec2(360.0, 120.0)); let pts = vec![r.left_top() + vec2(skew * 60.0, 0.0), r.right_top() + vec2(skew * 60.0, 0.0), r.right_bottom() - vec2(skew * 60.0, 0.0), r.left_bottom() - vec2(skew * 60.0, 0.0)]; p.add(Shape::convex_polygon(pts, ac, Stroke::NONE)); p.text(c, Align2::CENTER_CENTER, "滾快一點 → 傾斜", f(22.0), Color32::BLACK); }
        26 => { let items = ["Tokyo 東京", "Taipei 台北", "Paris 巴黎"]; for (j, s) in items.iter().enumerate() { let pos = pos2(a.left() + 80.0, a.top() + 30.0 + j as f32 * 55.0); let r = Rect::from_min_size(pos, vec2(300.0, 44.0)); let on = hover.map_or(false, |q| r.contains(q)); p.text(r.left_center(), Align2::LEFT_CENTER, *s, f(30.0), if on { ac } else { Color32::WHITE }); if on { let q = hover.unwrap(); p.rect_filled(Rect::from_center_size(q + vec2(200.0, 0.0), vec2(200.0, 130.0)), 8.0, hsv(j as f32 * 0.3)); p.text(q + vec2(200.0, 0.0), Align2::CENTER_CENTER, "🖼 image", f(20.0), Color32::BLACK); } } }
        _ => { for r in 0..3 { for col in 0..10 { let d = (r + col) as f32 * 0.07; let e = ease((t - d) / 0.5); let ctr = pos2(a.left() + 60.0 + col as f32 * 90.0, a.top() + 35.0 + r as f32 * 70.0); p.rect_filled(Rect::from_center_size(ctr, vec2(70.0, 55.0) * e), 6.0, hsv(d * 0.5)); } } }
    }
}

fn scene(p: &Painter, r: Rect, hue: f32, zoom: f32, pan: f32) {
    let pp = p.with_clip_rect(r);
    pp.rect_filled(r, 0.0, hsv(hue).gamma_multiply(0.6));
    let c = r.center() + vec2(pan, 0.0);
    pp.circle_filled(c + vec2(80.0, -40.0) * zoom, 28.0 * zoom, Color32::from_rgb(255, 230, 120));
    for k in 0..3 { let x = c.x + (k as f32 - 1.0) * 140.0 * zoom; let b = r.bottom(); pp.add(Shape::convex_polygon(vec![pos2(x - 120.0 * zoom, b), pos2(x, b - (90.0 + k as f32 * 20.0) * zoom), pos2(x + 120.0 * zoom, b)], hsv(hue + 0.3 + k as f32 * 0.05).gamma_multiply(0.8), Stroke::NONE)); }
}
fn rng(n: u32) -> f32 { let mut x = n.wrapping_mul(2654435761) ^ 0x9e3779b9; x ^= x >> 15; x = x.wrapping_mul(2246822519); x ^= x >> 13; (x % 10000) as f32 / 10000.0 }
fn star(c: Pos2, r: f32, n: usize) -> Vec<Pos2> { (0..n * 2).map(|k| { let a = k as f32 / (n * 2) as f32 * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2; let rr = if k % 2 == 0 { r } else { r * 0.45 }; c + vec2(a.cos(), a.sin()) * rr }).collect() }

fn draw3(k: usize, p: &Painter, a: Rect, t: f32, sp: f32, now: f32) {
    use std::f32::consts::{PI, TAU};
    let c = a.center();
    let f = |s: f32| FontId::proportional(s);
    let r = Rect::from_center_size(c, vec2(420.0, 200.0).min(a.size()));
    let pp = p.with_clip_rect(r);
    let cyc = (now % 4.0) / 4.0; // 0..1 looping
    let tri = ((cyc * TAU).sin() + 1.0) / 2.0;
    let txt = |s: &str, col: Color32| p.text(r.left_bottom() + vec2(0.0, 6.0), Align2::LEFT_TOP, s, f(14.0), col);
    let _ = (t, sp);
    match k {
        0 => { scene(p, r, 0.55, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::TRANSPARENT); let b = tri; let mut tmp = Vec::new(); tmp.push(0); let _ = tmp; let o = p.with_clip_rect(r); o.rect_filled(r, 0.0, Color32::from_black_alpha(0)); scene(&o.with_clip_rect(r), r, 0.05, 1.0, 0.0); o.rect_filled(r, 0.0, hsv(0.55).gamma_multiply(0.6 * (1.0 - b))); txt("A 場景與 B 場景重疊淡入淡出", Color32::GRAY); }
        1 => { scene(p, r, 0.1, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_black_alpha((tri * 255.0) as u8)); }
        2 => { let x = (cyc * 2.0).fract(); let off = if x < 0.5 { 0.0 } else { ease((x - 0.5) / 0.3) }; let shift = off * r.width(); scene(&pp, r.translate(vec2(-shift, 0.0)), 0.6, 1.0, 0.0); scene(&pp, r.translate(vec2(r.width() - shift, 0.0)), 0.05, 1.0, 0.0); if off > 0.05 && off < 0.95 { for j in 0..20 { let y = r.top() + j as f32 * 10.0; pp.line_segment([pos2(r.left(), y), pos2(r.right(), y)], Stroke::new(6.0, Color32::from_white_alpha(40))); } } }
        3 => { let z = 1.0 + (cyc * 3.0).powi(3); if cyc < 0.6 { scene(&pp, r, 0.6, z, 0.0); } else { scene(&pp, r, 0.0, 1.0 + (1.0 - (cyc - 0.6) / 0.4) * 2.0, 0.0); } }
        4 => { scene(&pp, r, 0.12, 1.0 + cyc * 0.6, -cyc * 80.0); txt("靜態照片緩慢推近＋平移", Color32::GRAY); }
        5 => { let speed = if (0.35..0.75).contains(&cyc) { 0.15 } else { 1.0 }; let x = (now * 220.0 * 0.0) + 0.0; let _ = x; let pos: f32 = { let mut s = 0.0f32; let steps = (cyc * 200.0) as i32; for i in 0..steps { let c2 = i as f32 / 200.0; s += if (0.35..0.75).contains(&c2) { 0.15 } else { 1.0 }; } s / 200.0 / 0.52 }; pp.rect_filled(r, 0.0, Color32::from_gray(30)); pp.circle_filled(pos2(r.left() + pos.min(1.0) * r.width(), c.y), 20.0, Color32::from_rgb(90, 200, 255)); p.text(r.center_top() + vec2(0.0, 8.0), Align2::CENTER_TOP, format!("speed ×{:.2}", speed), f(18.0), Color32::WHITE); }
        6 => { let fz = (0.4..0.75).contains(&cyc); let x = if fz { 0.4 } else { cyc }; scene(&pp, r, 0.5, 1.0, (x - 0.5) * 200.0); if fz { pp.rect_filled(r, 0.0, Color32::from_white_alpha(30)); pp.text(r.left_top() + vec2(12.0, 10.0), Align2::LEFT_TOP, "FREEZE — 主角介紹", f(24.0), Color32::YELLOW); } }
        7 => { let m = r.center().x; scene(&p.with_clip_rect(Rect::from_min_max(r.min, pos2(m - 2.0, r.bottom()))), r, 0.6, 1.0, -100.0); scene(&p.with_clip_rect(Rect::from_min_max(pos2(m + 2.0, r.top()), r.max)), r, 0.0, 1.0, 100.0); }
        8 => { scene(p, r, 0.55, 1.0, 0.0); let s = Rect::from_min_size(r.right_bottom() - vec2(150.0, 90.0) - vec2(10.0, 10.0), vec2(150.0, 90.0)); p.rect_filled(s.expand(3.0), 4.0, Color32::WHITE); scene(p, s, 0.95, 0.4, (now * 2.0).sin() * 20.0); }
        9 => { scene(p, r, 0.6, 1.0, 0.0); let x = r.left() + tri * r.width(); for j in 0..12 { pp.circle_filled(pos2(x, r.top() + 30.0), 160.0 - j as f32 * 12.0, Color32::from_rgba_unmultiplied(255, 140, 40, 10)); } }
        10 => { scene(p, r, 0.1, 1.0, 0.0); let s = (now * 24.0) as u32; for j in 0..900 { let x = rng(s * 1000 + j); let y = rng(s * 7777 + j * 3); pp.rect_filled(Rect::from_min_size(pos2(r.left() + x * r.width(), r.top() + y * r.height()), vec2(2.0, 2.0)), 0.0, if j % 2 == 0 { Color32::from_white_alpha(60) } else { Color32::from_black_alpha(80) }); } }
        11 => { scene(p, r, 0.1, 1.0, 0.0); let str = 0.4 + tri * 0.6; for j in 0..30 { let rr = r.width() * 0.25 + j as f32 * 8.0; for side in 0..1 { let _ = side; pp.circle_stroke(c, rr, Stroke::new(9.0, Color32::from_black_alpha((j as f32 * 7.0 * str).min(255.0) as u8))); } } }
        12 => { let o = 2.0 + tri * 6.0; for (d, col) in [(-o, Color32::from_rgba_unmultiplied(255, 0, 0, 160)), (o, Color32::from_rgba_unmultiplied(0, 120, 255, 160))] { p.text(c + vec2(d, 0.0), Align2::CENTER_CENTER, "CHROMA 色差", f(56.0), col); } p.text(c, Align2::CENTER_CENTER, "CHROMA 色差", f(56.0), Color32::from_rgba_unmultiplied(255, 255, 255, 200)); txt("越靠近邊緣偏移越大", Color32::GRAY); }
        13 => { let o = ((now * 9.0).sin() * 10.0).round(); scene(&p.with_clip_rect(r), r.translate(vec2(-o, 0.0)), 0.0, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(0, 255, 255, 40)); scene(&p.with_clip_rect(Rect::from_min_max(r.min, pos2(r.right(), r.top() + 60.0))), r.translate(vec2(o, 0.0)), 0.5, 1.0, 0.0); }
        14 => { scene(p, r, 0.08, 1.0, 0.0); let bh = r.height() * 0.15 * ease(t / 1.0); pp.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), bh)), 0.0, Color32::BLACK); pp.rect_filled(Rect::from_min_max(pos2(r.left(), r.bottom() - bh), r.max), 0.0, Color32::BLACK); txt("2.39:1 電影比例", Color32::GRAY); }
        15 => { let m = r.left() + tri * r.width(); scene(&p.with_clip_rect(Rect::from_min_max(r.min, pos2(m, r.bottom()))), r, 0.55, 1.0, 0.0); let right = p.with_clip_rect(Rect::from_min_max(pos2(m, r.top()), r.max)); scene(&right, r, 0.55, 1.0, 0.0); right.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(255, 120, 20, 70)); right.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), r.height() * 0.5)), 0.0, Color32::from_rgba_unmultiplied(0, 160, 160, 40)); p.line_segment([pos2(m, r.top()), pos2(m, r.bottom())], Stroke::new(2.0, Color32::WHITE)); txt("左：原始　右：Teal & Orange LUT", Color32::GRAY); }
        16 => { scene(p, r, 0.6, 1.0, 0.0); let e = ease(t / 0.8); let bar = Rect::from_min_size(pos2(r.left() + 16.0 - (1.0 - e) * 300.0, r.bottom() - 60.0), vec2(250.0, 42.0)); p.with_clip_rect(r).rect_filled(bar, 0.0, Color32::from_rgb(200, 40, 60)); p.with_clip_rect(r).text(bar.left_center() + vec2(10.0, 0.0), Align2::LEFT_CENTER, "柯盛達　導演 / Director", f(18.0), Color32::WHITE); }
        17 => { let words = ["MOVE", "動き", "BIG", "動態!"]; let j = ((now * 1.5) as usize) % 4; let lt = (now * 1.5).fract(); let s = 1.0 + (1.0 - ease(lt / 0.3)) * 1.5; let ang = if j % 2 == 0 { -0.1 } else { 0.08 }; let g = p.layout_no_wrap(words[j].into(), f(60.0 * s), Color32::WHITE); let _ = ang; p.galley(c - g.size() / 2.0, g, hsv(j as f32 * 0.2)); }
        18 => { let x = cyc; if x < 0.5 { pp.rect_filled(r, 0.0, Color32::from_gray(20)); pp.circle_filled(c, 50.0, Color32::from_rgb(240, 200, 120)); p.text(r.left_top() + vec2(8.0, 8.0), Align2::LEFT_TOP, "SHOT A：盤子", f(16.0), Color32::WHITE); } else { pp.rect_filled(r, 0.0, Color32::from_rgb(10, 10, 40)); pp.circle_filled(c, 50.0, Color32::from_rgb(240, 240, 220)); p.text(r.left_top() + vec2(8.0, 8.0), Align2::LEFT_TOP, "SHOT B：月亮（形狀相同）", f(16.0), Color32::WHITE); } }
        // ---- anime
        19 => { pp.rect_filled(r, 0.0, Color32::WHITE); for j in 0..90 { let an = j as f32 / 90.0 * TAU + rng(j + (now * 12.0) as u32) * 0.05; let r0 = 70.0 + rng(j * 3 + (now * 12.0) as u32) * 40.0; pp.add(Shape::convex_polygon(vec![c + vec2(an.cos(), an.sin()) * r0, c + vec2((an + 0.02).cos(), (an + 0.02).sin()) * 400.0, c + vec2((an - 0.02).cos(), (an - 0.02).sin()) * 400.0], Color32::BLACK, Stroke::NONE)); } p.text(c, Align2::CENTER_CENTER, "!!", f(48.0), Color32::RED); }
        20 => { let on = (now * 1.2).fract() < 0.15; if on { pp.rect_filled(r, 0.0, Color32::BLACK); pp.circle_filled(c, 70.0, Color32::WHITE); for j in 0..16 { let an = j as f32 / 16.0 * TAU; pp.line_segment([c, c + vec2(an.cos(), an.sin()) * 300.0], Stroke::new(4.0, Color32::WHITE)); } pp.circle_filled(c, 40.0, Color32::BLACK); } else { pp.rect_filled(r, 0.0, Color32::from_rgb(60, 70, 120)); pp.circle_filled(c, 30.0, Color32::from_rgb(255, 210, 170)); } }
        21 => { let x = r.left() + 40.0 + tri * (r.width() - 80.0); let v = ((cyc * TAU).cos()).abs(); pp.rect_filled(r, 0.0, Color32::from_gray(40)); for j in 0..8 { pp.circle_filled(pos2(x - j as f32 * 14.0 * v * (cyc * TAU).cos().signum(), c.y), 26.0 - j as f32 * 2.0, Color32::from_rgba_unmultiplied(255, 200, 120, (220 - j * 25) as u8)); } txt("快速動作時把角色拉長、殘影化", Color32::GRAY); }
        22 => { let sh = if (now * 0.8).fract() < 0.3 { vec2(rng((now * 60.0) as u32) - 0.5, rng((now * 60.0) as u32 + 9) - 0.5) * 24.0 } else { Vec2::ZERO }; scene(&pp, r.translate(sh), 0.75, 1.0, 0.0); p.text(c + sh, Align2::CENTER_CENTER, "ドン！", f(48.0), Color32::WHITE); }
        23 => { pp.rect_filled(r, 0.0, Color32::from_rgb(120, 170, 230)); for j in 0..30 { let x = r.left() + (rng(j) * r.width() + now * 30.0 + (now + j as f32).sin() * 20.0).rem_euclid(r.width()); let y = r.top() + (rng(j + 100) * r.height() + now * (30.0 + rng(j + 5) * 40.0)).rem_euclid(r.height()); quad(&pp, pos2(x, y), 6.0, 4.0, now * 2.0 + j as f32, 1.0, Color32::from_rgb(255, 183, 197)); } }
        24 => { let x = (now * 0.7).fract(); let s = if x < 0.2 { ease(x / 0.2) * 1.2 } else if x < 0.3 { 1.2 - (x - 0.2) * 2.0 } else { 1.0 }; pp.rect_filled(r, 0.0, Color32::from_rgb(255, 220, 230)); pp.circle_filled(c, 60.0 * s, Color32::from_rgb(255, 210, 170)); pp.circle_filled(c + vec2(-20.0, -5.0) * s, 6.0 * s, Color32::BLACK); pp.circle_filled(c + vec2(20.0, -5.0) * s, 6.0 * s, Color32::BLACK); pp.text(c + vec2(90.0, -50.0), Align2::CENTER_CENTER, "ポン♪", f(24.0), Color32::from_rgb(230, 80, 120)); }
        25 => { pp.rect_filled(r, 0.0, Color32::from_gray(230)); pp.circle_filled(c, 60.0, Color32::from_rgb(255, 210, 170)); let y = (now * 0.8).fract() * 30.0; let dp = c + vec2(55.0, -55.0 + y); pp.add(Shape::convex_polygon(vec![dp + vec2(0.0, -22.0), dp + vec2(10.0, 0.0), dp + vec2(0.0, 10.0), dp + vec2(-10.0, 0.0)], Color32::from_rgb(120, 190, 255), Stroke::new(2.0, Color32::from_rgb(40, 90, 200)))); }
        26 => { pp.rect_filled(r, 0.0, Color32::from_gray(230)); pp.circle_filled(c, 60.0, Color32::from_rgb(255, 210, 170)); let s = 1.0 + ((now * 8.0).sin() * 0.15); let m = c + vec2(50.0, -50.0); for q in 0..4 { let an = q as f32 * PI / 2.0 + PI / 4.0; let d = vec2(an.cos(), an.sin()); let o = m + d * 10.0 * s; let n = vec2(-d.y, d.x); pp.add(Shape::line(vec![o + n * 8.0 * s, o + d * 6.0 * s, o - n * 8.0 * s], Stroke::new(4.0, Color32::RED))); } }
        27 => { pp.rect_filled(r, 0.0, Color32::from_rgb(30, 20, 60)); for j in 0..6 { let ph = (now * 1.5 + j as f32 * 0.37).fract(); let s = (ph * PI).sin(); let pos = pos2(r.left() + rng(j * 11 + (now * 1.5 + j as f32 * 0.37) as u32) * r.width(), r.top() + rng(j * 7 + (now * 1.5 + j as f32 * 0.37) as u32) * r.height()); pp.add(Shape::convex_polygon(star(pos, 18.0 * s, 4), Color32::from_rgb(255, 250, 200), Stroke::NONE)); } }
        28 => { scene(p, r, 0.7, 1.0, 0.0); let x = (now * 0.6).fract(); let a2 = if x < 0.1 { x / 0.1 } else if x < 0.4 { 1.0 - (x - 0.1) / 0.3 } else { 0.0 }; pp.rect_filled(r, 0.0, Color32::from_white_alpha((a2 * 255.0) as u8)); }
        29 => { let w = a.width(); let pr = [Rect::from_min_size(a.min, vec2(w * 0.55, a.height() * 0.55)), Rect::from_min_max(pos2(a.left() + w * 0.57, a.top()), pos2(a.right(), a.top() + a.height() * 0.55)), Rect::from_min_max(pos2(a.left(), a.top() + a.height() * 0.58), a.max)]; for (j, q) in pr.iter().enumerate() { let e = ease(t / 0.5 - j as f32 * 0.6); if e <= 0.0 { continue; } p.rect_filled(q.shrink(2.0), 0.0, Color32::WHITE.gamma_multiply(e)); p.rect_stroke(q.shrink(2.0), 0.0, Stroke::new(3.0, Color32::BLACK)); p.text(q.center(), Align2::CENTER_CENTER, ["コマ1", "コマ2", "コマ3 ドーン"][j], f(22.0), Color32::BLACK.gamma_multiply(e)); } }
        30 => { let ang = 0.25 * tri - 0.05; let o = p.with_clip_rect(r); o.rect_filled(r, 0.0, Color32::from_rgb(60, 60, 80)); for j in 0..4 { quad(&o, c + vec2((j as f32 - 1.5) * 90.0, 30.0), 25.0, 70.0, ang, 1.0, Color32::from_gray(140)); } quad(&o, c + vec2(0.0, 100.0), 300.0, 20.0, ang, 1.0, Color32::from_gray(30)); txt("鏡頭傾斜→不安、緊張感", Color32::GRAY); }
        31 => { let o = (now * 120.0) % 200.0; pp.rect_filled(r, 0.0, Color32::from_rgb(150, 200, 255)); for j in -1..4 { let x = r.left() - o + j as f32 * 200.0; pp.rect_filled(Rect::from_min_size(pos2(x, r.bottom() - 80.0), vec2(60.0, 80.0)), 0.0, Color32::from_rgb(90, 140, 90)); pp.rect_filled(Rect::from_min_size(pos2(x + 100.0, r.bottom() - 120.0), vec2(50.0, 120.0)), 0.0, Color32::from_rgb(160, 120, 90)); } pp.circle_filled(pos2(c.x, r.bottom() - 30.0 - ((now * 10.0).sin().abs() * 8.0)), 22.0, Color32::from_rgb(255, 120, 120)); }
        32 => { let x = ease(t / 0.8); pp.rect_filled(r, 0.0, Color32::from_rgb(255, 240, 100)); let s = Rect::from_center_size(c, vec2(r.width(), 60.0 * x)); pp.rect_filled(s, 0.0, Color32::from_rgb(230, 60, 100)); p.text(c, Align2::CENTER_CENTER, "★ アイキャッチ ★  CM の後で", f(24.0 * x.max(0.1)), Color32::WHITE); }
        // ---- film
        33 => { scene(p, r, 0.1, 1.0, 0.0); let rr = (1.0 - tri) * 260.0; let ring = (0..60).map(|j| { let an = j as f32 / 60.0 * TAU; c + vec2(an.cos(), an.sin()) * (rr + 300.0) }); let _ = ring; pp.circle_stroke(c, rr + 250.0, Stroke::new(500.0, Color32::BLACK)); }
        34 => { scene(p, r, 0.6, 1.0, 0.0); let s = tri * 300.0 + 1.0; pp.add(Shape::convex_polygon(star(c, s, 5).into_iter().collect(), Color32::TRANSPARENT, Stroke::NONE)); let st = star(c, s, 5); for j in 0..10 { pp.add(Shape::convex_polygon(vec![c, st[j], st[(j + 1) % 10]], hsv(0.1).gamma_multiply(0.95), Stroke::NONE)); } txt("星形中露出下一個場景", Color32::GRAY); }
        35 => { scene(p, r, 0.0, 1.0, 0.0); let e = cyc * TAU; let n = 60; let mut pts = vec![c]; for j in 0..=n { let an = -PI / 2.0 + e * j as f32 / n as f32; pts.push(c + vec2(an.cos(), an.sin()) * 400.0); } for w in pts.windows(2).skip(1) { pp.add(Shape::convex_polygon(vec![c, w[0], w[1]], hsv(0.6).gamma_multiply(0.85), Stroke::NONE)); } }
        36 => { let focus_near = tri; for (j, (x, s, near)) in [(-90.0, 70.0, true), (90.0, 40.0, false)].iter().enumerate() { let blur = if *near { 1.0 - focus_near } else { focus_near } * 10.0; let col = if j == 0 { Color32::from_rgb(255, 160, 90) } else { Color32::from_rgb(90, 200, 255) }; for q in 0..6 { p.circle_filled(c + vec2(*x, 0.0), s + blur * q as f32 * 0.6, col.gamma_multiply(if blur < 1.0 { 1.0 } else { 0.18 })); } } txt("焦點在前景 ↔ 背景之間切換", Color32::GRAY); }
        37 => { let z = 0.6 + tri * 1.2; pp.rect_filled(r, 0.0, Color32::from_gray(20)); for j in 0..10 { let d = (j as f32 + 1.0) * 22.0 * z; pp.rect_stroke(Rect::from_center_size(c, vec2(d * 2.0, d * 1.2)), 0.0, Stroke::new(2.0, Color32::from_gray(150))); } pp.circle_filled(c, 26.0, Color32::from_rgb(255, 200, 160)); txt("主體大小不變、背景透視拉伸（希區考克）", Color32::GRAY); }
        38 => { pp.rect_filled(r, 0.0, Color32::from_rgb(10, 10, 25)); for j in 0..25 { let pos = pos2(r.left() + rng(j) * r.width(), r.top() + rng(j + 50) * r.height() + (now * 0.5 + j as f32).sin() * 5.0); pp.circle_filled(pos, 12.0 + rng(j + 9) * 22.0, hsv(rng(j + 3) * 0.2 + 0.05).gamma_multiply(0.35)); } }
        39 => { pp.rect_filled(r, 0.0, Color32::from_rgb(20, 20, 40)); let l = pos2(r.left() + 40.0 + tri * 200.0, r.top() + 40.0); let d = c - l; for j in 0..8 { pp.circle_filled(l, 60.0 - j as f32 * 7.0, Color32::from_rgba_unmultiplied(255, 220, 160, 20)); } for (j, k2) in [0.5, 0.9, 1.3, 1.7].iter().enumerate() { pp.circle_stroke(l + d * *k2 * 2.0, 10.0 + j as f32 * 8.0, Stroke::new(3.0, hsv(j as f32 * 0.2).gamma_multiply(0.5))); } pp.line_segment([l - vec2(200.0, 0.0), l + vec2(200.0, 0.0)], Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 220, 160, 90))); }
        40 => { scene(p, r, 0.6, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_black_alpha(90)); for j in 0..120 { let x = r.left() + rng(j) * (r.width() + 60.0); let y = r.top() + (rng(j + 500) * r.height() + now * 600.0 * (0.7 + rng(j + 3) * 0.6)).rem_euclid(r.height()); pp.line_segment([pos2(x, y), pos2(x - 4.0, y + 14.0)], Stroke::new(1.0, Color32::from_rgba_unmultiplied(200, 220, 255, 140))); } }
        41 => { scene(p, r, 0.3, 1.0, 0.0); for j in 0..(r.height() as i32 / 3) { pp.line_segment([pos2(r.left(), r.top() + j as f32 * 3.0), pos2(r.right(), r.top() + j as f32 * 3.0)], Stroke::new(1.0, Color32::from_black_alpha(90))); } let y = r.top() + ((now * 60.0) % r.height()); pp.rect_filled(Rect::from_min_size(pos2(r.left(), y), vec2(r.width(), 10.0)), 0.0, Color32::from_white_alpha(25)); p.text(r.left_top() + vec2(10.0, 8.0), Align2::LEFT_TOP, "▶ PLAY  SP 0:12:34", FontId::monospace(16.0), Color32::WHITE); }
        42 => { scene(p, r, 0.1, 1.0, 0.0); let burn = (now * 0.5).fract(); if burn > 0.6 { let b = (burn - 0.6) / 0.4; pp.circle_filled(r.right_top() + vec2(-40.0, 40.0), b * 260.0, Color32::from_rgba_unmultiplied(255, 120, 20, 200)); pp.circle_filled(r.right_top() + vec2(-40.0, 40.0), b * 160.0, Color32::from_rgba_unmultiplied(255, 255, 230, 230)); } if (now * 5.0).sin() > 0.9 { pp.rect_filled(Rect::from_min_size(pos2(r.left(), c.y), vec2(r.width(), 18.0)), 0.0, Color32::from_rgba_unmultiplied(255, 0, 100, 120)); } }
        43 => { let pq = p.with_clip_rect(a); let lines = ["DIRECTED BY", "柯盛達", "", "MUSIC", "Grok Bot", "", "STARRING", "Rust 🦀", "egui", "", "THE END 劇終"]; for (j, l) in lines.iter().enumerate() { let y = a.bottom() - ((now * 40.0) % (a.height() + 400.0)) + j as f32 * 36.0; pq.text(pos2(c.x, y), Align2::CENTER_CENTER, *l, f(if j % 3 == 0 { 16.0 } else { 26.0 }), Color32::WHITE); } }
        44 => { let n = 8 - ((now % 8.0) as i32); let ph = now.fract(); pp.rect_filled(r, 0.0, Color32::from_gray(170)); pp.circle_stroke(c, 80.0, Stroke::new(3.0, Color32::WHITE)); pp.circle_stroke(c, 64.0, Stroke::new(2.0, Color32::WHITE)); pp.line_segment([pos2(r.left(), c.y), pos2(r.right(), c.y)], Stroke::new(2.0, Color32::BLACK)); pp.line_segment([pos2(c.x, r.top()), pos2(c.x, r.bottom())], Stroke::new(2.0, Color32::BLACK)); let mut pts = vec![c]; for j in 0..=40 { let an = -PI / 2.0 + ph * TAU * j as f32 / 40.0; pts.push(c + vec2(an.cos(), an.sin()) * 300.0); } for w in pts.windows(2).skip(1) { pp.add(Shape::convex_polygon(vec![c, w[0], w[1]], Color32::from_black_alpha(70), Stroke::NONE)); } p.text(c, Align2::CENTER_CENTER, n.to_string(), f(90.0), Color32::BLACK); }
        _ => { let fl = 0.8 + rng((now * 24.0) as u32) * 0.2; scene(p, r, 0.1, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(110, 80, 40, 120)); pp.rect_filled(r, 0.0, Color32::from_black_alpha(((1.0 - fl) * 600.0) as u8)); let fr = (now * 24.0) as u32; for j in 0..3 { if rng(fr + j * 13) > 0.5 { let x = r.left() + rng(fr * 3 + j) * r.width(); pp.line_segment([pos2(x, r.top()), pos2(x + 3.0, r.bottom())], Stroke::new(1.0, Color32::from_white_alpha(120))); } } if rng(fr + 999) > 0.8 { pp.circle_filled(pos2(r.left() + rng(fr) * r.width(), r.top() + rng(fr + 1) * r.height()), 4.0, Color32::BLACK); } }
    }
}
