use eframe::emath::TSTransform;
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

fn ease(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); 1.0 - (1.0 - t).powi(4) }
#[allow(dead_code)]
fn spring(t: f32) -> f32 { if t <= 0.0 { 0.0 } else { 1.0 - (-6.0 * t).exp() * (12.0 * t).cos() } }

/// smoothing factor for frame-rate independent exponential easing
fn damp(rate: f32, dt: f32) -> f32 { 1.0 - (-rate * dt).exp() }
fn smoothstep(x: f32) -> f32 { let x = x.clamp(0.0, 1.0); x * x * (3.0 - 2.0 * x) }

struct App { start: HashMap<usize, f64>, cur: usize, follow: Pos2, vel: f32, tops: Vec<f32>, jump: Option<usize>, cat_y: [f32; 5], open: Option<(usize, f64)>, hover_amt: HashMap<usize, f32> }

const BG: Color32 = Color32::from_rgb(9, 8, 26);
const STAGE: Color32 = Color32::from_rgb(14, 14, 36);
const CARD: Color32 = Color32::from_rgb(24, 24, 52);
const ACCENT: Color32 = Color32::from_rgb(150, 140, 255);
const ACCENT2: Color32 = Color32::from_rgb(110, 220, 240);
const PINK: Color32 = Color32::from_rgb(250, 140, 205);
const MINT: Color32 = Color32::from_rgb(140, 240, 200);
const AMBER: Color32 = Color32::from_rgb(255, 205, 150);
const TEXT: Color32 = Color32::from_rgb(240, 238, 255);
const MUTED: Color32 = Color32::from_rgb(170, 168, 210);
const CATS: [(usize, &str, &str, &str, &str); 5] = [
    (0, "🌐", "網頁滾動動畫", "Web Scroll", "スクロール演出"),
    (17, "✨", "互動與微動畫", "Micro-interactions", "マイクロインタラクション"),
    (45, "🎬", "影片剪輯效果", "Video Editing", "映像編集"),
    (64, "🌸", "動畫／漫畫效果", "Anime", "アニメ演出"),
    (78, "🎞", "電影轉場與鏡頭", "Cinema", "映画技法"),
];
const VW: f32 = 900.0; // virtual demo width the demo code was designed for

fn grad_rect(p: &Painter, r: Rect, c1: Color32, c2: Color32) {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), c1); m.colored_vertex(r.right_top(), c2);
    m.colored_vertex(r.right_bottom(), c2); m.colored_vertex(r.left_bottom(), c1);
    m.add_triangle(0, 1, 2); m.add_triangle(0, 2, 3);
    p.add(Shape::mesh(m));
}
/// soft radial glow: opaque-ish center fading to transparent rim
fn radial(p: &Painter, c: Pos2, r: Vec2, col: Color32) {
    let mut m = Mesh::default();
    m.colored_vertex(c, col);
    let n = 48;
    for k in 0..=n { let a = k as f32 / n as f32 * std::f32::consts::TAU; m.colored_vertex(c + vec2(a.cos() * r.x, a.sin() * r.y), Color32::TRANSPARENT); }
    for k in 1..=n as u32 { m.add_triangle(0, k, k + 1); }
    p.add(Shape::mesh(m));
}
fn glow_text(p: &Painter, pos: Pos2, al: Align2, s: &str, size: f32, col: Color32, glow: Color32) {
    for (k, rr) in [10.0f32, 6.0, 3.0].iter().enumerate() {
        for j in 0..8 { let a = j as f32 / 8.0 * std::f32::consts::TAU; p.text(pos + vec2(a.cos(), a.sin()) * *rr, al, s, FontId::proportional(size), glow.gamma_multiply(0.035 + k as f32 * 0.03)); }
    }
    p.text(pos, al, s, FontId::proportional(size), col);
}
fn glass(p: &Painter, r: Rect, round: f32, glow: f32, tint: Color32) {
    p.add(epaint::Shadow { offset: vec2(0.0, 14.0), blur: 40.0, spread: 0.0, color: Color32::from_black_alpha(120) }.as_shape(r, round));
    if glow > 0.01 { p.add(epaint::Shadow { offset: Vec2::ZERO, blur: 36.0, spread: 2.0, color: tint.gamma_multiply(0.45 * glow) }.as_shape(r, round)); }
    p.rect_filled(r, round, Color32::from_rgba_unmultiplied(40, 36, 90, 120));
    p.line_segment([r.left_top() + vec2(round, 1.0), r.right_top() + vec2(-round, 1.0)], Stroke::new(1.0, Color32::from_white_alpha(50)));
    p.rect_stroke(r, round, Stroke::new(1.0 + glow, Color32::from_rgba_unmultiplied(200, 190, 255, (40.0 + 120.0 * glow) as u8)));
}

fn theme(ctx: &Context) {
    let mut v = Visuals::dark();
    v.panel_fill = Color32::TRANSPARENT; v.window_fill = CARD; v.extreme_bg_color = STAGE;
    v.override_text_color = Some(TEXT);
    for w in [&mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active] { w.rounding = Rounding::same(20.0); w.bg_stroke = Stroke::new(1.0, Color32::from_white_alpha(30)); }
    v.widgets.inactive.weak_bg_fill = Color32::from_white_alpha(14);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgba_unmultiplied(150, 140, 255, 60);
    v.widgets.active.weak_bg_fill = Color32::from_rgba_unmultiplied(150, 140, 255, 100);
    ctx.set_visuals(v);
    let mut st = (*ctx.style()).clone();
    st.spacing.button_padding = vec2(14.0, 6.0);
    st.spacing.scroll = egui::style::ScrollStyle::floating();
    ctx.set_style(st);
}

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 840.0]).with_title("Anime Showcase — 動態效果圖鑑"), ..Default::default() };
    eframe::run_native("anime-showcase", opts, Box::new(|cc| {
        let mut f = FontDefinitions::default();
        f.font_data.insert("cjk".into(), FontData::from_static(include_bytes!("../assets/NotoSansCJK-Regular.ttc")));
        for fam in [FontFamily::Proportional, FontFamily::Monospace] { f.families.get_mut(&fam).unwrap().push("cjk".into()); }
        cc.egui_ctx.set_fonts(f);
        cc.egui_ctx.set_theme(Theme::Dark);
        theme(&cc.egui_ctx);
        let args: Vec<String> = std::env::args().collect();
        let num = |flag: &str| args.iter().skip_while(|a| *a != flag).nth(1).and_then(|n| n.parse::<usize>().ok()).filter(|n| (1..=SECTIONS.len()).contains(n)).map(|n| n - 1);
        let (jump, open) = (num("--goto"), num("--open").map(|i| (i, 0.0)));
        Ok(Box::new(App { start: HashMap::new(), cur: 0, follow: Pos2::ZERO, vel: 0.0, tops: Vec::new(), jump, cat_y: [0.0; 5], open, hover_amt: HashMap::new() }))
    }))
}

fn paint_bg(p: &Painter, r: Rect, now: f32, off: f32) {
    p.rect_filled(r, 0.0, BG);
    let blobs = [(0.15, 0.25, ACCENT, 0.9), (0.85, 0.2, PINK, 0.7), (0.5, 0.75, ACCENT2, 0.55), (0.2, 0.85, Color32::from_rgb(120, 80, 220), 0.8), (0.9, 0.7, Color32::from_rgb(90, 140, 255), 0.6)];
    for (k, (x, y, col, a)) in blobs.iter().enumerate() {
        let k = k as f32;
        let c = pos2(r.left() + r.width() * (x + 0.08 * (now * 0.07 + k * 1.7).sin()), r.top() + r.height() * (y + 0.06 * (now * 0.05 + k).cos()) - (off * 0.04 * (1.0 + k * 0.3)).rem_euclid(r.height() * 0.0 + 1.0) * 0.0 - (off * 0.03 * (k + 1.0)) % (r.height() * 0.4));
        radial(p, c, vec2(r.width() * 0.45, r.height() * 0.5), col.gamma_multiply(0.22 * a));
    }
    // aurora ribbons
    for band in 0..3 {
        let b = band as f32; let mut m = Mesh::default(); let n = 60;
        let col = [ACCENT2, PINK, MINT][band].gamma_multiply(0.07);
        for j in 0..=n { let x = r.left() + r.width() * j as f32 / n as f32; let y = r.top() + r.height() * (0.3 + 0.15 * b) + ((x * 0.006) + now * 0.25 + b * 2.0).sin() * 60.0 - (off * 0.02) % 200.0;
            m.colored_vertex(pos2(x, y - 90.0), Color32::TRANSPARENT); m.colored_vertex(pos2(x, y), col); m.colored_vertex(pos2(x, y + 50.0), Color32::TRANSPARENT); }
        for j in 0..n as u32 { let i = j * 3; for (a, bb, c) in [(i, i + 1, i + 3), (i + 1, i + 4, i + 3), (i + 1, i + 2, i + 4), (i + 2, i + 5, i + 4)] { m.add_triangle(a, bb, c); } }
        p.add(Shape::mesh(m));
    }
    // floating sparkles / bokeh with depth parallax
    for j in 0..140u32 {
        let depth = 0.2 + rng(j + 3) * 0.8;
        let x = r.left() + rng(j) * r.width() + (now * 0.3 + j as f32).sin() * 12.0 * depth;
        let y = r.top() + (rng(j + 77) * r.height() * 1.2 - off * 0.15 * depth - now * 6.0 * depth).rem_euclid(r.height() * 1.2) - r.height() * 0.1;
        let tw = 0.5 + 0.5 * (now * (1.0 + rng(j + 9) * 2.0) + j as f32).sin();
        if j % 9 == 0 { radial(p, pos2(x, y), Vec2::splat(18.0 + 30.0 * depth), [PINK, ACCENT2, ACCENT][j as usize % 3].gamma_multiply(0.12 * tw)); }
        else { p.circle_filled(pos2(x, y), 0.6 + 1.6 * depth, Color32::from_white_alpha((40.0 + 170.0 * tw * depth) as u8)); }
        if j % 23 == 0 { let s = 5.0 * tw * depth + 1.0; let c = pos2(x, y); let col = Color32::from_white_alpha((200.0 * tw) as u8); p.line_segment([c - vec2(s * 2.0, 0.0), c + vec2(s * 2.0, 0.0)], Stroke::new(1.0, col)); p.line_segment([c - vec2(0.0, s * 2.0), c + vec2(0.0, s * 2.0)], Stroke::new(1.0, col)); }
    }
    // vignette
    radial(p, pos2(r.center().x, r.top() - 40.0), vec2(r.width() * 0.6, 140.0), Color32::from_rgba_unmultiplied(4, 3, 14, 120));
}

struct Card { i: usize, rect: Rect }

fn layout_cards(x0: f32, w: f32, y: &mut f32, ids: std::ops::Range<usize>, gap: f32) -> Vec<Card> {
    let mut out = Vec::new();
    let items: Vec<usize> = ids.collect();
    let mut k = 0;
    let patterns: [&[f32]; 4] = [&[2.0, 1.0], &[1.0, 1.0, 1.0], &[1.0, 2.0], &[1.0, 1.0, 1.0]];
    let mut row = 0;
    let unit = (w - gap * 2.0) / 3.0;
    while k < items.len() {
        let (spans, h): (Vec<f32>, f32) = if k == 0 { (vec![3.0], 440.0) } else { let pt = patterns[row % 4]; row += 1; (pt.to_vec(), if pt.len() == 2 { 380.0 } else { 340.0 }) };
        let n = spans.len().min(items.len() - k);
        let spans: Vec<f32> = if n < spans.len() { vec![3.0 / n as f32; n] } else { spans };
        let mut x = x0;
        for s in spans.iter() {
            let cw = unit * s + gap * (s - 1.0);
            out.push(Card { i: items[k], rect: Rect::from_min_size(pos2(x, *y), vec2(cw, h)) });
            x += cw + gap; k += 1;
        }
        *y += h + gap;
    }
    out
}

impl App {
    /// paint a demo into its own transformed layer so it scales cleanly to any card size
    #[allow(clippy::too_many_arguments)]
    fn demo(&self, ctx: &Context, order: Order, key: (usize, u8), i: usize, stage: Rect, clip: Rect, t: f32, sp: f32, now: f32, ptr: Option<Pos2>, follow: Option<Pos2>, vel: f32) {
        let s = (stage.width() / VW).clamp(0.5, 1.4);
        let vs = stage.size() / s;
        let layer = LayerId::new(order, Id::new(("demo", key)));
        let tr = TSTransform::new(stage.min.to_vec2(), s);
        ctx.set_transform_layer(layer, tr);
        let inv = tr.inverse();
        let varea = Rect::from_min_size(Pos2::ZERO, vs);
        let vclip = inv.mul_rect(clip.intersect(stage)).intersect(varea);
        let p = ctx.layer_painter(layer).with_clip_rect(vclip);
        p.rect_filled(varea, 18.0 / s, Color32::from_rgba_unmultiplied(10, 10, 30, 200));
        let map = |q: Option<Pos2>| q.map(|q| inv * q);
        draw(i, &p, varea.shrink(14.0), t, sp, now, map(follow), map(ptr), vel);
        // re-round the corners: paint the card glass colour over square demo corners
        let rr = 18.0 / s; let cc = Color32::from_rgb(24, 22, 58);
        for (corner, dir) in [(varea.left_top(), vec2(1.0, 1.0)), (varea.right_top(), vec2(-1.0, 1.0)), (varea.right_bottom(), vec2(-1.0, -1.0)), (varea.left_bottom(), vec2(1.0, -1.0))] {
            let c0 = corner + dir * rr; let mut pts = vec![corner];
            for j in 0..=8 { let a = std::f32::consts::FRAC_PI_2 * j as f32 / 8.0; let v = vec2(-a.cos() * dir.x, -a.sin() * dir.y) * rr; pts.push(c0 + v); }
            let _ = cc; p.add(Shape::convex_polygon(pts, Color32::from_rgb(26, 24, 60), Stroke::NONE));
        }
        p.rect_stroke(varea, rr, Stroke::new(1.0 / s, Color32::from_white_alpha(25)));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        let nowf = now as f32;
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        let ptr = ctx.input(|i| i.pointer.hover_pos());
        if let Some(q) = ptr { self.follow += (q - self.follow) * damp(12.0, dt); }
        let raw_vel = ctx.input(|i| i.smooth_scroll_delta.y);
        self.vel += (raw_vel - self.vel) * damp(8.0, dt);
        let (follow, vel) = (ptr.map(|_| self.follow), self.vel);
        let screen = ctx.screen_rect();
        let prev_off = ctx.data(|d| d.get_temp::<f32>(Id::new("off"))).unwrap_or(0.0);
        paint_bg(&ctx.layer_painter(LayerId::background()), screen, nowf, prev_off);
        if ctx.input(|i| i.key_pressed(Key::Escape)) { self.open = None; }
        let modal = self.open.is_some();

        let mut clicked: Option<usize> = None;
        CentralPanel::default().frame(Frame::none()).show(ctx, |ui| {
            let view = ui.max_rect();
            let mut sa = ScrollArea::vertical().auto_shrink(false);
            if let Some(y) = self.jump.and_then(|j| self.tops.get(j).copied()) { sa = sa.vertical_scroll_offset((y - 110.0).max(0.0)); self.jump = None; }
            if let Some(k) = ctx.data(|d| d.get_temp::<usize>(Id::new("catjump"))) { sa = sa.vertical_scroll_offset(self.cat_y[k]); ctx.data_mut(|d| d.remove::<usize>(Id::new("catjump"))); }
            if modal { sa = sa.enable_scrolling(false); }
            let out = sa.show(ui, |ui| {
                let top0 = ui.min_rect().top();
                let full = ui.available_width();
                let w = (full - 96.0).min(1180.0);
                let x0 = ui.min_rect().left() + (full - w) / 2.0;
                let p = ui.painter().clone();
                let off = view.top() - top0;
                // ---------- HERO ----------
                let hero_h = view.height();
                let hc = pos2(view.center().x, top0 + hero_h * 0.46 + off * 0.45);
                let fade = (1.0 - off / (hero_h * 0.7)).clamp(0.0, 1.0);
                radial(&p, hc, vec2(520.0, 260.0), ACCENT.gamma_multiply(0.25 * fade));
                radial(&p, hc + vec2(160.0 * (nowf * 0.4).sin(), 40.0), vec2(300.0, 160.0), PINK.gamma_multiply(0.18 * fade));
                p.text(hc - vec2(0.0, 118.0), Align2::CENTER_CENTER, "M O T I O N   A T L A S", FontId::proportional(15.0), MUTED.gamma_multiply(fade));
                glow_text(&p, hc - vec2(0.0, 40.0), Align2::CENTER_CENTER, "夢幻動態圖鑑", 84.0, TEXT.gamma_multiply(fade), PINK.gamma_multiply(fade));
                p.text(hc + vec2(0.0, 40.0), Align2::CENTER_CENTER, "モーション図鑑  ·  The Dreamy Motion Encyclopedia", FontId::proportional(20.0), Color32::from_rgb(205, 200, 255).gamma_multiply(fade));
                p.text(hc + vec2(0.0, 78.0), Align2::CENTER_CENTER, "91 種網頁・影片・動畫・電影效果，每一個都附上 中文／English／日本語 名稱", FontId::proportional(15.0), MUTED.gamma_multiply(fade));
                let pills = ["91 effects", "5 chapters", "3 languages"];
                for (k, s) in pills.iter().enumerate() { let c = hc + vec2((k as f32 - 1.0) * 150.0, 132.0); let r = Rect::from_center_size(c, vec2(130.0, 36.0)); p.rect_filled(r, 18.0, Color32::from_white_alpha((14.0 * fade) as u8)); p.rect_stroke(r, 18.0, Stroke::new(1.0, Color32::from_white_alpha((50.0 * fade) as u8))); p.text(c, Align2::CENTER_CENTER, *s, FontId::proportional(14.0), TEXT.gamma_multiply(fade)); }
                let bob = (nowf * 2.0).sin() * 6.0;
                p.text(pos2(view.center().x, top0 + hero_h - 70.0 + bob), Align2::CENTER_CENTER, "scroll  ↓", FontId::proportional(14.0), MUTED.gamma_multiply(fade));
                let mut y = top0 + hero_h;
                self.tops = vec![0.0; SECTIONS.len()];
                let gap = 26.0;
                for (k, c) in CATS.iter().enumerate() {
                    // ---------- chapter divider ----------
                    let end = CATS.get(k + 1).map(|x| x.0).unwrap_or(SECTIONS.len());
                    y += 40.0;
                    let dh = 300.0;
                    self.cat_y[k] = y - top0 + 20.0;
                    let dc = pos2(view.center().x, y + dh / 2.0);
                    let par = (dc.y - view.center().y) * 0.15;
                    radial(&p, dc + vec2(0.0, par), vec2(w * 0.55, 170.0), [ACCENT, ACCENT2, PINK, Color32::from_rgb(255, 160, 200), AMBER][k].gamma_multiply(0.2));
                    p.text(dc + vec2(0.0, par * 2.0 - 10.0), Align2::CENTER_CENTER, c.3.to_uppercase(), FontId::proportional(150.0), Color32::from_white_alpha(4));
                    p.text(dc - vec2(0.0, 86.0), Align2::CENTER_CENTER, format!("CHAPTER  0{}   ·   {} effects", k + 1, end - c.0), FontId::proportional(13.0), MUTED);
                    glow_text(&p, dc - vec2(0.0, 22.0), Align2::CENTER_CENTER, &format!("{}  {}", c.1, c.2), 54.0, TEXT, [ACCENT, ACCENT2, PINK, PINK, AMBER][k]);
                    p.text(dc + vec2(0.0, 36.0), Align2::CENTER_CENTER, format!("{}  ·  {}", c.3, c.4), FontId::proportional(18.0), Color32::from_rgb(205, 200, 255));
                    let lw = 220.0; grad_rect(&p, Rect::from_center_size(dc + vec2(0.0, 72.0), vec2(lw, 2.0)), Color32::TRANSPARENT, PINK); grad_rect(&p, Rect::from_min_size(dc + vec2(0.0, 71.0), vec2(lw / 2.0, 2.0)), PINK, Color32::TRANSPARENT);
                    grad_rect(&p, Rect::from_min_size(dc + vec2(-lw / 2.0, 71.0), vec2(lw / 2.0, 2.0)), Color32::TRANSPARENT, PINK);
                    y += dh;
                    // ---------- bento grid ----------
                    for card in layout_cards(x0, w, &mut y, c.0..end, gap) {
                        let i = card.i; let s = SECTIONS[i];
                        self.tops[i] = card.rect.top() - top0;
                        let base = card.rect;
                        let vis = base.intersects(view);
                        if base.top() <= view.top() + view.height() * 0.4 { self.cur = i; }
                        if base.top() < view.bottom() - 60.0 && base.bottom() > view.top() { self.start.entry(i).or_insert(now); } else if !vis { self.start.remove(&i); }
                        if !vis { continue; }
                        let resp = ui.interact(base, Id::new(("card", i)), Sense::click());
                        let hov = resp.hovered() && !modal;
                        if resp.clicked() && !modal { clicked = Some(i); }
                        let h = self.hover_amt.entry(i).or_insert(0.0); *h += ((if hov { 1.0 } else { 0.0 }) - *h) * damp(10.0, dt); let ha = *h;
                        let float = (nowf * 0.9 + i as f32 * 0.7).sin() * 3.0 - ha * 8.0;
                        let tilt = if hov { ptr.map(|q| (q - base.center()) / base.size()).unwrap_or(Vec2::ZERO) } else { Vec2::ZERO };
                        let rect = base.translate(vec2(tilt.x * 6.0 * ha, float + tilt.y * 4.0 * ha)).expand(ha * 4.0);
                        let tint = [ACCENT, ACCENT2, PINK, PINK, AMBER][k];
                        glass(&p, rect, 26.0, 0.25 + ha * 0.75, tint);
                        let big = base.height() > 400.0;
                        // header: number pill, title, subtitles
                        let pill = Rect::from_min_size(rect.min + vec2(22.0, 22.0), vec2(44.0, 26.0));
                        p.rect_filled(pill, 13.0, tint.gamma_multiply(0.25)); p.rect_stroke(pill, 13.0, Stroke::new(1.0, tint.gamma_multiply(0.7)));
                        p.text(pill.center(), Align2::CENTER_CENTER, s.0, FontId::proportional(13.0), TEXT);
                        if big { p.text(pill.right_center() + vec2(12.0, 0.0), Align2::LEFT_CENTER, "FEATURED ★", FontId::proportional(12.0), tint); }
                        p.text(rect.min + vec2(24.0, 60.0), Align2::LEFT_TOP, s.3, FontId::proportional(if big { 32.0 } else { 22.0 }), TEXT);
                        p.text(rect.min + vec2(24.0, if big { 102.0 } else { 92.0 }), Align2::LEFT_TOP, format!("{}  ·  {}", s.2, s.1), FontId::proportional(13.0), MUTED);
                        if ha > 0.05 { p.text(rect.right_top() + vec2(-24.0, 30.0), Align2::RIGHT_CENTER, "點擊放大 ＋", FontId::proportional(13.0), TEXT.gamma_multiply(ha)); }
                        let stage = Rect::from_min_max(rect.min + vec2(18.0, if big { 132.0 } else { 120.0 }), rect.max - vec2(18.0, 18.0));
                        let sp = ((view.bottom() - base.top()) / (view.height() + base.height())).clamp(0.0, 1.0);
                        let t = self.start.get(&i).map(|s0| (now - s0) as f32).unwrap_or(0.0);
                        self.demo(ctx, Order::Middle, (i, 0), i, stage, view, t, sp, nowf, if modal { None } else { ptr }, if modal { None } else { follow }, vel);
                    }
                }
                y += 120.0;
                p.text(pos2(view.center().x, y), Align2::CENTER_CENTER, "★  fin  ·  おしまい  ·  完  ★", FontId::proportional(16.0), MUTED);
                y += 140.0;
                ui.allocate_space(vec2(full, y - top0));
            });
            let off = out.state.offset.y;
            ctx.data_mut(|d| d.insert_temp(Id::new("off"), off));
            let max = (out.content_size.y - out.inner_rect.height()).max(1.0);
            let lp = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("prog")));
            grad_rect(&lp, Rect::from_min_size(view.min, vec2(view.width() * off / max, 3.0)), ACCENT2, PINK);
        });

        // ---------- floating glass nav ----------
        let curcat = CATS.iter().rposition(|c| self.cur >= c.0).unwrap_or(0);
        Area::new(Id::new("nav")).order(Order::Foreground).fixed_pos(pos2(screen.center().x, 18.0)).pivot(Align2::CENTER_TOP).show(ctx, |ui| {
            Frame::none().fill(Color32::from_rgba_unmultiplied(28, 24, 64, 200)).stroke(Stroke::new(1.0, Color32::from_white_alpha(40))).rounding(28.0).inner_margin(Margin::symmetric(16.0, 8.0))
                .shadow(epaint::Shadow { offset: vec2(0.0, 8.0), blur: 30.0, spread: 0.0, color: Color32::from_black_alpha(140) }).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("★ 夢幻動態圖鑑").size(15.0).strong().color(Color32::from_rgb(220, 210, 255)));
                    ui.add_space(10.0);
                    for (k, c) in CATS.iter().enumerate() {
                        let on = k == curcat;
                        let b = Button::new(RichText::new(format!("{} {}", c.1, c.3)).size(13.0).color(if on { TEXT } else { MUTED })).fill(if on { Color32::from_rgba_unmultiplied(150, 140, 255, 70) } else { Color32::TRANSPARENT }).stroke(Stroke::NONE).rounding(16.0);
                        if ui.add(b).clicked() { ctx.data_mut(|d| d.insert_temp(Id::new("catjump"), k)); self.open = None; }
                    }
                    ui.add_space(6.0);
                    if ui.add(Button::new(RichText::new("⟲").size(15.0)).rounding(16.0)).on_hover_text("Replay 重播").clicked() { self.start.clear(); }
                });
            });
        });
        // now-playing chip (sticky)
        let s = SECTIONS[self.cur];
        Area::new(Id::new("now")).order(Order::Foreground).fixed_pos(pos2(24.0, screen.bottom() - 24.0)).pivot(Align2::LEFT_BOTTOM).show(ctx, |ui| {
            Frame::none().fill(Color32::from_rgba_unmultiplied(28, 24, 64, 190)).stroke(Stroke::new(1.0, Color32::from_white_alpha(35))).rounding(20.0).inner_margin(Margin::symmetric(14.0, 8.0)).show(ui, |ui| {
                ui.label(RichText::new(format!("📌 {}  {}  ·  {}", s.0, s.3, s.2)).size(12.0).color(Color32::from_rgb(215, 210, 255)));
            });
        });

        if let Some(i) = clicked { self.open = Some((i, now)); }
        // ---------- detail modal ----------
        if let Some((i, t0)) = self.open {
            let e = ease(((now - t0) as f32) / 0.35);
            let lp = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("backdrop")));
            lp.rect_filled(screen, 0.0, Color32::from_rgba_unmultiplied(6, 4, 20, (190.0 * e) as u8));
            let mr = Rect::from_center_size(screen.center() + vec2(0.0, (1.0 - e) * 40.0), vec2((screen.width() - 120.0).min(1100.0), (screen.height() - 120.0).min(720.0)) * (0.94 + 0.06 * e));
            let top = LayerId::new(Order::Tooltip, Id::new("modal"));
            let mp = ctx.layer_painter(top);
            glass(&mp, mr, 30.0, 1.0, PINK);
            mp.rect_filled(mr, 30.0, Color32::from_rgba_unmultiplied(20, 16, 48, 200));
            let s = SECTIONS[i];
            mp.text(mr.min + vec2(36.0, 30.0), Align2::LEFT_TOP, format!("No. {}", s.0), FontId::proportional(14.0), PINK);
            glow_text(&mp, mr.min + vec2(36.0, 56.0), Align2::LEFT_TOP, s.3, 44.0, TEXT, PINK);
            mp.text(mr.min + vec2(38.0, 118.0), Align2::LEFT_TOP, format!("English  ·  {}", s.2), FontId::proportional(18.0), Color32::from_rgb(205, 200, 255));
            mp.text(mr.min + vec2(38.0, 146.0), Align2::LEFT_TOP, format!("日本語  ·  {}", s.1), FontId::proportional(18.0), Color32::from_rgb(205, 200, 255));
            let close = Rect::from_center_size(mr.right_top() + vec2(-44.0, 44.0), vec2(40.0, 40.0));
            mp.circle_filled(close.center(), 20.0, Color32::from_white_alpha(20)); mp.text(close.center(), Align2::CENTER_CENTER, "×", FontId::proportional(18.0), TEXT);
            mp.text(mr.right_top() + vec2(-80.0, 44.0), Align2::RIGHT_CENTER, "Esc 關閉", FontId::proportional(12.0), MUTED);
            let stage = Rect::from_min_max(mr.min + vec2(30.0, 186.0), mr.max - vec2(30.0, 30.0));
            let t = (now - t0) as f32;
            self.demo(ctx, Order::Debug, (i, 1), i, stage, screen, t, 0.5 + 0.5 * (nowf * 0.4).sin(), nowf, ptr, follow, vel);
            let (pressed, pos) = ctx.input(|inp| (inp.pointer.primary_clicked(), inp.pointer.interact_pos()));
            if pressed && (now - t0) > 0.2 { if let Some(q) = pos { if close.contains(q) || !mr.contains(q) { self.open = None; } } }
        }
        ctx.request_repaint();
    }
}

#[allow(clippy::too_many_arguments)]
fn draw(i: usize, p: &Painter, a: Rect, t: f32, sp: f32, now: f32, ptr: Option<Pos2>, raw: Option<Pos2>, vel: f32) {
    let accent = ACCENT;
    let c = a.center();
    let card = |p: &Painter, r: Rect, alpha: f32| { p.add(epaint::Shadow { offset: vec2(0.0, 8.0), blur: 20.0, spread: 0.0, color: Color32::from_black_alpha((90.0 * alpha) as u8) }.as_shape(r, 14.0)); grad_rect(&p.with_clip_rect(r.intersect(p.clip_rect())), r, ACCENT.gamma_multiply(alpha), PINK.gamma_multiply(alpha)); };
    match i {
        0 => { let e = ease(t / 1.2); card(p, Rect::from_center_size(c, vec2(320.0, 140.0)), e); p.text(c, Align2::CENTER_CENTER, "opacity 0 → 1", FontId::proportional(24.0), Color32::BLACK.gamma_multiply(e)); }
        1 => { let e = ease(t / 1.0); card(p, Rect::from_center_size(c + vec2(-300.0 * (1.0 - e), 0.0), vec2(320.0, 140.0)), e); }
        2 => for k in 0..6 { let e = ease((t - k as f32 * 0.15) / 0.7); let x = a.left() + 20.0 + k as f32 * 150.0; card(p, Rect::from_min_size(pos2(x, a.top() + 40.0 + 40.0 * (1.0 - e)), vec2(130.0, 130.0)), e); },
        3 => { let s: Vec<char> = "一個字一個字地出現 Text reveal!".chars().collect(); let n = ((t * 12.0) as usize).min(s.len()); let txt: String = s[..n].iter().collect(); p.text(c, Align2::CENTER_CENTER, txt, FontId::proportional(40.0), Color32::WHITE); }
        4 => { let e = ease(t / 1.4); let r = Rect::from_center_size(c, vec2(600.0, 160.0)); let clip = Rect::from_min_max(r.min, pos2(r.left() + r.width() * e, r.bottom())); let pp = p.with_clip_rect(clip); pp.rect_filled(r, 10.0, PINK); pp.text(r.center(), Align2::CENTER_CENTER, "clip-path: inset()", FontId::proportional(36.0), Color32::BLACK); }
        5 => { let e = ease(t / 1.5); let blur = (1.0 - e) * 14.0; for k in 0..8 { let o = (k as f32 - 3.5) / 3.5 * blur; p.text(c + vec2(o, 0.0), Align2::CENTER_CENTER, "Blur-in 模糊淡入", FontId::proportional(48.0), Color32::WHITE.gamma_multiply(e / if blur > 0.5 { 4.0 } else { 1.0 })); } }
        6 => { let e = ease(t / 2.0); for (k, (v, u)) in [(12800.0, "users"), (98.0, "%"), (365.0, "days")].iter().enumerate() { p.text(pos2(a.left() + 150.0 + k as f32 * 300.0, c.y), Align2::CENTER_CENTER, format!("{:.0} {}", v * e, u), FontId::proportional(44.0), accent); } }
        7 => for (k, v) in [0.9, 0.7, 0.55].iter().enumerate() { let e = ease((t - k as f32 * 0.2) / 1.5) * v; let y = a.top() + 30.0 + k as f32 * 55.0; p.rect_filled(Rect::from_min_size(pos2(a.left(), y), vec2(a.width(), 26.0)), 13.0, Color32::from_rgb(36, 42, 66)); p.rect_filled(Rect::from_min_size(pos2(a.left(), y), vec2(a.width() * e, 26.0)), 13.0, accent); p.text(pos2(a.right() - 8.0, y + 13.0), Align2::RIGHT_CENTER, format!("{:.0}%", e * 100.0), FontId::proportional(16.0), Color32::WHITE); },
        8 => { p.rect_filled(Rect::from_min_size(pos2(a.left(), c.y), vec2(a.width(), 12.0)), 6.0, Color32::from_rgb(36, 42, 66)); p.rect_filled(Rect::from_min_size(pos2(a.left(), c.y), vec2(a.width() * sp, 12.0)), 6.0, accent); p.text(pos2(a.left(), c.y - 40.0), Align2::LEFT_CENTER, format!("跟著捲動位置變化 progress = {:.2}（頂部藍條為全頁進度）", sp), FontId::proportional(20.0), Color32::WHITE); }
        9 => { let pp = p.with_clip_rect(a); for (k, &(_r, depth)) in [(0.2f32, 60.0f32), (0.6, 160.0), (1.2, 300.0)].iter().enumerate() { let y = a.bottom() - (sp - 0.5) * depth - 60.0 - k as f32 * 30.0; for j in 0..5 { pp.circle_filled(pos2(a.left() + 80.0 + j as f32 * 200.0 + k as f32 * 60.0, y), 14.0 + k as f32 * 10.0, accent.gamma_multiply(0.3 + 0.3 * k as f32)); } } p.text(a.left_top(), Align2::LEFT_TOP, "遠景慢、近景快", FontId::proportional(18.0), MUTED); }
        10 => { let s = 0.4 + 1.0 * sp; card(p, Rect::from_center_size(c, vec2(260.0 * s, 110.0 * s)).intersect(a), 1.0); p.text(c, Align2::CENTER_CENTER, format!("scale {:.2}", s), FontId::proportional(24.0), Color32::BLACK); }
        11 => { let f = (sp * 23.99) as usize; let ang = f as f32 / 24.0 * std::f32::consts::TAU; p.circle_stroke(c, 80.0, Stroke::new(3.0, MUTED)); p.line_segment([c, c + vec2(ang.cos(), ang.sin()) * 80.0], Stroke::new(8.0, accent)); p.text(pos2(c.x + 140.0, c.y), Align2::LEFT_CENTER, format!("frame {:02}/24", f + 1), FontId::proportional(24.0), Color32::WHITE); }
        12 => { let e = ease(t / 2.5); let n = 200; let pts: Vec<Pos2> = (0..=((n as f32 * e) as usize)).map(|k| { let x = k as f32 / n as f32; pos2(a.left() + 40.0 + x * (a.width() - 80.0), c.y + (x * 12.0).sin() * 60.0 * (1.0 - x * 0.5)) }).collect(); p.add(Shape::line(pts, Stroke::new(4.0, accent))); }
        13 => { let txt = "Scroll to fill 捲動填色"; let font = FontId::proportional(56.0); let g = p.layout_no_wrap(txt.into(), font.clone(), Color32::from_rgb(55, 62, 92)); let r = Align2::CENTER_CENTER.anchor_size(c, g.size()); p.galley(r.min, g, Color32::WHITE); let fill = ((sp - 0.25) * 2.0).clamp(0.0, 1.0); let pp = p.with_clip_rect(Rect::from_min_max(r.min, pos2(r.left() + r.width() * fill, r.bottom()))); pp.text(c, Align2::CENTER_CENTER, txt, font, Color32::WHITE); }
        14 => { let pp = p.with_clip_rect(a); let y0 = a.top() + 40.0 - sp * 420.0; for k in 0..10 { let r = Rect::from_min_size(pos2(a.left() + 40.0, y0 + k as f32 * 44.0), vec2(a.width() * 0.5, 36.0)); pp.rect_filled(r, 6.0, Color32::from_rgb(38, 45, 70)); pp.text(r.left_center() + vec2(12.0, 0.0), Align2::LEFT_CENTER, format!("content row {}", k + 1), FontId::proportional(16.0), MUTED); } let hd = Rect::from_min_size(pos2(a.left() + 40.0, a.top()), vec2(a.width() * 0.5, 34.0)); pp.rect_filled(hd, 6.0, Color32::from_rgb(60, 140, 90)); pp.text(hd.center(), Align2::CENTER_CENTER, "📌 position: sticky; top: 0", FontId::proportional(18.0), Color32::WHITE); p.text(pos2(a.right() - 20.0, c.y), Align2::RIGHT_CENTER, "內容隨捲動上移，
標題釘在頂端
（右上角 📌 亦同）", FontId::proportional(20.0), MINT); }
        15 => { let pp = p.with_clip_rect(a); let (cw, gap) = (220.0f32, 20.0); let step = (now / 2.0).floor(); let lt = now % 2.0; let x = if lt < 0.7 { step + 0.65 * ease(lt / 0.7) } else { let u = lt - 0.7; step + 1.0 - 0.35 * (-u * 6.0).exp() * (u * 14.0).cos() }; let frame = Rect::from_center_size(c, vec2(cw + 8.0, 150.0)); for k in 0..5 { let mut d = (k as f32 - x).rem_euclid(5.0); if d > 2.5 { d -= 5.0; } let cx = c.x + d * (cw + gap); pp.rect_filled(Rect::from_center_size(pos2(cx, c.y), vec2(cw, 130.0)), 12.0, hsv(k as f32 * 0.12)); pp.text(pos2(cx, c.y), Align2::CENTER_CENTER, format!("Slide {}", k + 1), FontId::proportional(24.0), Color32::BLACK); } p.rect_stroke(frame, 14.0, Stroke::new(2.0, Color32::WHITE)); p.text(a.left_bottom(), Align2::LEFT_BOTTOM, "scroll-snap-type: x mandatory — 放手後彈回最近的卡片（亦可勾選上方「Scroll snap」）", FontId::proportional(15.0), MUTED); }
        16 => { let pp = p.with_clip_rect(a); let txt = "MARQUEE ✦ 跑馬燈 ✦ INFINITE LOOP ✦ マーキー ✦ "; let g = pp.layout_no_wrap(txt.into(), FontId::proportional(40.0), accent); let w = g.size().x; let off = (now * 120.0) % w; for k in 0..4 { pp.galley(pos2(a.left() - off + k as f32 * w, c.y - 20.0), g.clone(), accent); } }
        45.. => draw3(i - 45, p, a, t, sp, now),
        _ => draw2(i - 17, p, a, t, sp, now, ptr, raw, vel),
    }
}

fn rot(c: Pos2, v: Vec2, a: f32) -> Pos2 { c + vec2(v.x * a.cos() - v.y * a.sin(), v.x * a.sin() + v.y * a.cos()) }
fn quad(p: &Painter, c: Pos2, w: f32, h: f32, ang: f32, sx: f32, col: Color32) {
    let pts = [vec2(-w, -h), vec2(w, -h), vec2(w, h), vec2(-w, h)].iter().map(|v| rot(c, vec2(v.x * sx, v.y), ang)).collect();
    p.add(Shape::convex_polygon(pts, col, Stroke::NONE));
}
/// fill a star-shaped (w.r.t. `c`) outline as one triangle-fan mesh: handles concave shapes, no AA seams
fn fan(p: &Painter, c: Pos2, pts: &[Pos2], col: Color32) {
    let mut m = epaint::Mesh::default(); m.colored_vertex(c, col);
    for q in pts { m.colored_vertex(*q, col); }
    let n = pts.len() as u32; for k in 0..n { m.add_triangle(0, 1 + k, 1 + (k + 1) % n); }
    p.add(m);
}
/// filled pie slice from angle `a0` sweeping `sweep` radians
fn sector(p: &Painter, c: Pos2, r: f32, a0: f32, sweep: f32, col: Color32) {
    let n = 64; let mut m = epaint::Mesh::default(); m.colored_vertex(c, col);
    for j in 0..=n { let an = a0 + sweep * j as f32 / n as f32; m.colored_vertex(c + vec2(an.cos(), an.sin()) * r, col); }
    for j in 0..n { m.add_triangle(0, 1 + j, 2 + j); }
    p.add(m);
}
fn hsv(h: f32) -> Color32 { let x = h.rem_euclid(1.0); let hue = 0.55 + 0.42 * (x * std::f32::consts::TAU).sin().abs(); ecolor::Hsva::new(hue.rem_euclid(1.0), 0.48, 0.97, 1.0).into() }

#[allow(clippy::too_many_arguments)]
fn draw2(k: usize, p: &Painter, a: Rect, t: f32, sp: f32, now: f32, ptr: Option<Pos2>, raw: Option<Pos2>, vel: f32) {
    use std::f32::consts::{PI, TAU};
    let ac = ACCENT;
    let c = a.center();
    let f = |s: f32| FontId::proportional(s);
    let pp = p.with_clip_rect(a);
    let hover = ptr.filter(|q| a.contains(*q));
    let loop_t = t % 3.0;
    match k {
        0 => { let e = ease(t / 0.8); quad(p, c, 160.0 * (0.3 + 0.7 * e), 70.0 * (0.3 + 0.7 * e), 0.0, 1.0, ac.gamma_multiply(e)); }
        1 => { let e = ease(t / 1.0); quad(p, c, 160.0, 70.0, (1.0 - e) * -PI / 2.0, 1.0, ac.gamma_multiply(e)); quad(p, c + vec2(300.0, 0.0), 80.0, 70.0, 0.0, (e * PI / 2.0).sin().max(0.02), PINK); }
        2 => { let x = loop_t / 1.5; let y = if x < 1.0 { let b = 1.0 - x; (b * PI * 3.0).sin().abs() * b * b } else { 0.0 }; p.circle_filled(pos2(c.x, a.bottom() - 30.0 - y * 150.0), 30.0, ac); }
        3 => { let x = loop_t; let s = 1.0 + (-(x * 4.0)).exp() * (x * 18.0).sin() * 0.6; quad(p, c, 90.0 * s, 70.0 * (2.0 - s), 0.0, 1.0, ac); }
        4 => { let s = "> Hello, typewriter_ 打字機"; let ch: Vec<char> = s.chars().collect(); let n = ((t * 10.0) as usize).min(ch.len()); let mut out: String = ch[..n].iter().collect(); if (now * 2.0) as i32 % 2 == 0 { out.push('▌'); } p.text(pos2(a.left() + 40.0, c.y), Align2::LEFT_CENTER, out, FontId::monospace(36.0), MINT); }
        5 => { let tgt: Vec<char> = "DECODING 解碼完成".chars().collect(); let pool: Vec<char> = "!<>-_\\/[]{}=+*^?#ABXZ01".chars().collect(); let out: String = tgt.iter().enumerate().map(|(j, &ch)| if t * 8.0 > j as f32 + 4.0 { ch } else { pool[((now * 30.0) as usize + j * 7) % pool.len()] }).collect(); p.text(c, Align2::CENTER_CENTER, out, FontId::monospace(44.0), Color32::WHITE); }
        6 => { let txt = "Gradient Shimmer 流光文字"; let g = p.layout_no_wrap(txt.into(), f(52.0), Color32::WHITE); let r = Align2::CENTER_CENTER.anchor_size(c, g.size()); let n = 40; for j in 0..n { let x0 = r.left() + r.width() * j as f32 / n as f32; let cl = Rect::from_min_max(pos2(x0, r.top()), pos2(x0 + r.width() / n as f32 + 1.0, r.bottom())); p.with_clip_rect(cl).galley(r.min, g.clone(), hsv(j as f32 / n as f32 * 0.5 - now * 0.4)); } }
        7 => { let txt: Vec<char> = "WAVE TEXT 波浪文字".chars().collect(); let mut x = a.left() + 120.0; for (j, ch) in txt.iter().enumerate() { let y = (now * 5.0 - j as f32 * 0.5).sin() * 20.0; let g = p.layout_no_wrap(ch.to_string(), f(48.0), Color32::WHITE); let w = g.size().x; p.galley(pos2(x, c.y - 24.0 + y), g, hsv(j as f32 * 0.05)); x += w; } }
        8 => { let r = Rect::from_center_size(c, vec2(320.0, 160.0)); let infl = ptr.map_or(0.0, |q| smoothstep(1.0 - (q - c).length() / 360.0)); let (dx, dy) = ptr.map_or((0.0, 0.0), |q| (((q.x - c.x) / 300.0).clamp(-1.0, 1.0) * infl, ((q.y - c.y) / 100.0).clamp(-1.0, 1.0) * infl)); let lift = 10.0 * infl; p.rect_filled(r.translate(vec2(0.0, 12.0)), 12.0, Color32::from_black_alpha(120)); let pts: Vec<Pos2> = [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom()].iter().map(|q| { let v = *q - c; c + vec2(v.x * (1.0 + dy * v.y.signum() * 0.08), v.y * (1.0 - dx * v.x.signum() * 0.08)) - vec2(0.0, lift) }).collect(); p.add(Shape::convex_polygon(pts, ac, Stroke::NONE)); p.text(c - vec2(0.0, lift), Align2::CENTER_CENTER, "滑鼠移上來試試", f(22.0), Color32::BLACK); }
        9 => { let off = ptr.map_or(Vec2::ZERO, |q| { let d = q - c; d * 0.35 * smoothstep(1.0 - d.length() / 260.0) }); p.rect_filled(Rect::from_center_size(c + off, vec2(200.0, 64.0)), 32.0, ac); p.text(c + off, Align2::CENTER_CENTER, "Magnetic 磁吸", f(22.0), Color32::BLACK); }
        10 => { p.rect_filled(Rect::from_center_size(c, vec2(360.0, 120.0)), 12.0, Color32::from_rgb(40, 47, 74)); for j in 0..3 { let x = (loop_t - j as f32 * 0.5).max(0.0); pp.circle_stroke(c, x * 120.0, Stroke::new(3.0, ac.gamma_multiply((1.0 - x / 2.0).max(0.0)))); } }
        11 => { let ph = (now * 0.5) % 3.0; let (s0, m) = (ph as usize, ease((ph.fract() - 0.4) / 0.6)); let n = 120; let shape_r = |kind: usize, ang: f32| -> f32 { match kind { 0 => 90.0, 1 => { let u = vec2(ang.cos(), ang.sin()); 80.0 / u.x.abs().max(u.y.abs()) } _ => { let q = (ang + PI / 2.0).rem_euclid(TAU / 5.0) / (TAU / 5.0); 45.0 + 60.0 * (1.0 - 2.0 * (q - 0.5).abs()).powi(2) } } }; let pts: Vec<Pos2> = (0..n).map(|j| { let ang = j as f32 / n as f32 * TAU; let rr = shape_r(s0, ang) * (1.0 - m) + shape_r((s0 + 1) % 3, ang) * m; c + vec2(ang.cos(), ang.sin()) * rr }).collect(); fan(p, c, &pts, hsv((s0 as f32 + m) * 0.15).gamma_multiply(0.85)); p.add(Shape::closed_line(pts, Stroke::new(3.0, Color32::WHITE))); p.text(pos2(a.right() - 20.0, c.y), Align2::RIGHT_CENTER, "● → ■ → ★", f(22.0), MUTED); }
        12 => { let x = -(sp - 0.2).clamp(0.0, 0.6) / 0.6 * 1400.0; for j in 0..8 { let r = Rect::from_min_size(pos2(a.left() + x + j as f32 * 260.0, a.top() + 20.0), vec2(240.0, 170.0)); pp.rect_filled(r, 10.0, hsv(j as f32 * 0.1)); pp.text(r.center(), Align2::CENTER_CENTER, format!("Panel {}", j + 1), f(26.0), Color32::BLACK); } }
        13 => { let e = ease(t / 1.2); pp.text(c, Align2::CENTER_CENTER, "Curtain 布幕拉開", f(48.0), Color32::WHITE); let hw = a.width() / 2.0 * (1.0 - e); pp.rect_filled(Rect::from_min_size(a.min, vec2(hw, a.height())), 0.0, Color32::from_rgb(98, 52, 140)); pp.rect_filled(Rect::from_min_max(pos2(a.right() - hw, a.top()), a.max), 0.0, Color32::from_rgb(98, 52, 140)); }
        14 => { for j in 0..3 { let r = Rect::from_min_size(pos2(a.left() + 120.0, a.top() + 20.0 + j as f32 * 50.0), vec2(a.width() - 240.0 - j as f32 * 120.0, 30.0)); p.rect_filled(r, 8.0, Color32::from_rgb(38, 45, 70)); let sx = r.left() + ((now * 0.8) % 1.5) * r.width() - 100.0; p.with_clip_rect(r).rect_filled(Rect::from_min_size(pos2(sx, r.top()), vec2(100.0, 30.0)), 0.0, Color32::from_rgb(58, 66, 98)); } p.circle_filled(pos2(a.left() + 60.0, a.top() + 45.0), 30.0, Color32::from_rgb(38, 45, 70)); }
        15 => { let x0 = a.left() + 120.0; let n = 64; let arc: Vec<Pos2> = (0..n).map(|j| { let ang = now * 5.0 + j as f32 / n as f32 * 4.5; pos2(x0, c.y) + vec2(ang.cos(), ang.sin()) * 40.0 }).collect(); p.add(Shape::line(arc, Stroke::new(6.0, ac))); for j in 0..3 { let s = ((now * 5.0 - j as f32).sin() + 1.0) / 2.0; p.circle_filled(pos2(x0 + 200.0 + j as f32 * 40.0, c.y), 8.0 + s * 6.0, ac); } for j in 0..8 { let ang = j as f32 / 8.0 * TAU; let al = ((j as f32 - now * 10.0).rem_euclid(8.0)) / 8.0; p.line_segment([pos2(x0 + 480.0, c.y) + vec2(ang.cos(), ang.sin()) * 20.0, pos2(x0 + 480.0, c.y) + vec2(ang.cos(), ang.sin()) * 40.0], Stroke::new(5.0, Color32::WHITE.gamma_multiply(al))); } }
        16 => { let x = loop_t / 1.5; for j in 0..40 { let ang = j as f32 * 2.4; let sp2 = 80.0 + (j * 37 % 100) as f32 * 1.5; let d = vec2(ang.cos(), ang.sin()) * sp2 * x + vec2(0.0, 120.0 * x * x); pp.circle_filled(c + d, 5.0 * (1.0 - x).max(0.0), hsv(j as f32 * 0.03)); } }
        17 => { let txt = "GLITCH 故障"; let gl = ((now * 7.0).sin() * (now * 13.0).sin()) > 0.6; let o = if gl { 6.0 } else { 0.0 }; p.text(c + vec2(-o, 0.0), Align2::CENTER_CENTER, txt, f(64.0), Color32::from_rgba_unmultiplied(255, 0, 80, 180)); p.text(c + vec2(o, 0.0), Align2::CENTER_CENTER, txt, f(64.0), Color32::from_rgba_unmultiplied(0, 255, 255, 180)); p.text(c, Align2::CENTER_CENTER, txt, f(64.0), Color32::WHITE); if gl { pp.rect_filled(Rect::from_min_size(pos2(a.left(), c.y + ((now * 50.0) % 60.0) - 30.0), vec2(a.width(), 6.0)), 0.0, Color32::from_black_alpha(200)); } }
        18 => { let s = ((now * 4.0).sin() + 1.0) / 2.0; p.circle_filled(c, 40.0 + 30.0 * s, ac.gamma_multiply(0.3 * (1.0 - s))); p.circle_filled(c, 40.0, ac); }
        19 => { let x = (now % 2.0).min(0.6); let dx = (x * 40.0).sin() * 15.0 * (1.0 - x / 0.6); p.rect_filled(Rect::from_center_size(c + vec2(dx, 0.0), vec2(320.0, 56.0)), 8.0, Color32::from_rgb(225, 85, 120)); p.text(c + vec2(dx, 0.0), Align2::CENTER_CENTER, "密碼錯誤 Wrong password", f(20.0), Color32::WHITE); }
        20 => { for j in 0..3 { let open = ease((t - 0.6 - j as f32 * 0.6) / 0.4) * if j == 1 { 1.0 } else { 0.0 }; let y = a.top() + j as f32 * 50.0 + if j == 2 { open * 0.0 + ease((t - 1.2) / 0.4) * 60.0 } else { 0.0 }; p.rect_filled(Rect::from_min_size(pos2(a.left() + 100.0, y), vec2(a.width() - 200.0, 42.0)), 6.0, Color32::from_rgb(40, 47, 74)); p.text(pos2(a.left() + 120.0, y + 21.0), Align2::LEFT_CENTER, format!("{} 問題 Q{}", if j == 1 && t > 1.2 { "▼" } else { "▶" }, j + 1), f(18.0), Color32::WHITE); if j == 1 { let h = ease((t - 1.2) / 0.4) * 60.0; pp.rect_filled(Rect::from_min_size(pos2(a.left() + 100.0, y + 44.0), vec2(a.width() - 200.0, h)), 6.0, Color32::from_rgb(30, 36, 58)); let _ = open; } } }
        21 => { let ang = ease(((t - 0.5) / 1.0).clamp(0.0, 1.0)) * PI; let sx = ang.cos(); let front = sx > 0.0; quad(p, c, 110.0, 80.0, 0.0, sx.abs().max(0.02), if front { ac } else { Color32::from_rgb(250, 180, 80) }); p.text(c, Align2::CENTER_CENTER, if front { "FRONT 正面" } else { "BACK 背面" }, f(20.0), Color32::BLACK); }
        22 => { let e = ease(t / 2.0) * 0.75; p.circle_stroke(c, 70.0, Stroke::new(12.0, Color32::from_rgb(38, 45, 70))); let n = 80; let pts: Vec<Pos2> = (0..=n).map(|j| { let ang = -PI / 2.0 + TAU * e * j as f32 / n as f32; c + vec2(ang.cos(), ang.sin()) * 70.0 }).collect(); p.add(Shape::line(pts, Stroke::new(12.0, ac))); p.text(c, Align2::CENTER_CENTER, format!("{:.0}%", e * 100.0), f(30.0), Color32::WHITE); }
        23 => { for (row, dir) in [(0.0, 1.0), (1.0, -1.0)] { let txt = if dir > 0.0 { "← LEFT 向左 ✦ " } else { "RIGHT 向右 → ✦ " }; let g = pp.layout_no_wrap(txt.into(), f(36.0), ac); let w = g.size().x; let off = (now * 100.0 * dir).rem_euclid(w); for j in -1..8 { pp.galley(pos2(a.left() - off + j as f32 * w, a.top() + 30.0 + row * 70.0), g.clone(), if dir > 0.0 { ac } else { PINK }); } } }
        24 => { if let (Some(q), Some(real)) = (hover, raw.filter(|q| a.contains(*q))) { p.line_segment([q, real], Stroke::new(1.0, ac.gamma_multiply(0.3))); p.circle_stroke(q, 24.0, Stroke::new(2.0, ac)); p.circle_filled(real, 5.0, Color32::WHITE); } else { p.text(c, Align2::CENTER_CENTER, "把滑鼠移進來，圓圈會跟著游標", f(22.0), MUTED); } }
        25 => { let skew = (vel * 0.02).clamp(-0.5, 0.5); let r = Rect::from_center_size(c, vec2(360.0, 120.0)); let pts = vec![r.left_top() + vec2(skew * 60.0, 0.0), r.right_top() + vec2(skew * 60.0, 0.0), r.right_bottom() - vec2(skew * 60.0, 0.0), r.left_bottom() - vec2(skew * 60.0, 0.0)]; p.add(Shape::convex_polygon(pts, ac, Stroke::NONE)); p.text(c, Align2::CENTER_CENTER, "滾快一點 → 傾斜", f(22.0), Color32::BLACK); }
        26 => { let items = ["Tokyo 東京", "Taipei 台北", "Paris 巴黎"]; for (j, s) in items.iter().enumerate() { let pos = pos2(a.left() + 80.0, a.top() + 30.0 + j as f32 * 55.0); let r = Rect::from_min_size(pos, vec2(300.0, 44.0)); let on = raw.is_some_and(|q| r.contains(q)); p.text(r.left_center(), Align2::LEFT_CENTER, *s, f(30.0), if on { ac } else { Color32::WHITE }); if let (true, Some(q)) = (on, ptr) { p.rect_filled(Rect::from_center_size(q + vec2(200.0, 0.0), vec2(200.0, 130.0)), 8.0, hsv(j as f32 * 0.3)); p.text(q + vec2(200.0, 0.0), Align2::CENTER_CENTER, "🖼 image", f(20.0), Color32::BLACK); } } }
        _ => { for r in 0..3 { for col in 0..10 { let d = (r + col) as f32 * 0.07; let e = ease((t - d) / 0.5); let ctr = pos2(a.left() + 60.0 + col as f32 * 90.0, a.top() + 35.0 + r as f32 * 70.0); p.rect_filled(Rect::from_center_size(ctr, vec2(70.0, 55.0) * e), 6.0, hsv(d * 0.5)); } } }
    }
}

fn scene(p: &Painter, r: Rect, hue: f32, zoom: f32, pan: f32) { scene_a(p, r, hue, zoom, pan, 1.0) }
/// same placeholder shot drawn at opacity `al` (for dissolves)
fn scene_a(p: &Painter, r: Rect, hue: f32, zoom: f32, pan: f32, al: f32) {
    let pp = p.with_clip_rect(r);
    pp.rect_filled(r, 14.0, hsv(hue).gamma_multiply(0.6 * al));
    let c = r.center() + vec2(pan, 0.0);
    pp.circle_filled(c + vec2(80.0, -40.0) * zoom, 28.0 * zoom, Color32::from_rgb(255, 214, 165).gamma_multiply(al));
    for k in 0..3 { let x = c.x + (k as f32 - 1.0) * 140.0 * zoom; let b = r.bottom(); pp.add(Shape::convex_polygon(vec![pos2(x - 120.0 * zoom, b), pos2(x, b - (90.0 + k as f32 * 20.0) * zoom), pos2(x + 120.0 * zoom, b)], hsv(hue + 0.3 + k as f32 * 0.05).gamma_multiply(0.8 * al), Stroke::NONE)); }
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
    let _ = sp;
    match k {
        0 => { pp.rect_filled(r, 0.0, Color32::BLACK); scene(&pp, r, 0.55, 1.0, -cyc * 40.0); scene_a(&pp, r, 0.05, 1.2, 60.0 - cyc * 60.0, tri); txt(&format!("A 場景與 B 場景重疊淡入淡出　B opacity {:.2}", tri), MUTED); }
        1 => { scene(p, r, 0.1, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_black_alpha((tri * 255.0) as u8)); }
        2 => { let x = (cyc * 2.0).fract(); let off = if x < 0.5 { 0.0 } else { ease((x - 0.5) / 0.3) }; let shift = off * r.width(); scene(&pp, r.translate(vec2(-shift, 0.0)), 0.6, 1.0, 0.0); scene(&pp, r.translate(vec2(r.width() - shift, 0.0)), 0.05, 1.0, 0.0); if off > 0.05 && off < 0.95 { for j in 0..20 { let y = r.top() + j as f32 * 10.0; pp.line_segment([pos2(r.left(), y), pos2(r.right(), y)], Stroke::new(6.0, Color32::from_white_alpha(40))); } } }
        3 => { let z = 1.0 + (cyc * 3.0).powi(3); if cyc < 0.6 { scene(&pp, r, 0.6, z, 0.0); } else { scene(&pp, r, 0.0, 1.0 + (1.0 - (cyc - 0.6) / 0.4) * 2.0, 0.0); } }
        4 => { scene(&pp, r, 0.12, 1.0 + cyc * 0.6, -cyc * 80.0); txt("靜態照片緩慢推近＋平移", MUTED); }
        5 => { let speed = if (0.35..0.75).contains(&cyc) { 0.15 } else { 1.0 }; let dist = |u: f32| u.min(0.35) + (u.clamp(0.35, 0.75) - 0.35) * 0.15 + (u - 0.75).max(0.0); let pos = dist(cyc) / dist(1.0); for g in 1..6 { let gx = r.left() + (pos - g as f32 * 0.02 * speed).clamp(0.0, 1.0) * r.width(); pp.circle_filled(pos2(gx, c.y), 20.0, ACCENT2.gamma_multiply(0.12)); } pp.rect_filled(r, 0.0, STAGE); pp.circle_filled(pos2(r.left() + pos * r.width(), c.y), 20.0, ACCENT2); p.text(r.center_top() + vec2(0.0, 8.0), Align2::CENTER_TOP, format!("speed ×{:.2}", speed), f(18.0), Color32::WHITE); }
        6 => { let fz = (0.4..0.75).contains(&cyc); let x = if fz { 0.4 } else { cyc }; scene(&pp, r, 0.5, 1.0, (x - 0.5) * 200.0); if fz { pp.rect_filled(r, 0.0, Color32::from_white_alpha(30)); pp.text(r.left_top() + vec2(12.0, 10.0), Align2::LEFT_TOP, "FREEZE — 主角介紹", f(24.0), AMBER); } }
        7 => { let e = ease(t / 0.9); let m = r.center().x; let sl = (1.0 - e) * r.width() * 0.5; pp.rect_filled(r, 0.0, Color32::BLACK); scene(&pp.with_clip_rect(Rect::from_min_max(r.min, pos2(m - 2.0, r.bottom()))), r.translate(vec2(-sl, 0.0)), 0.6, 1.0, -100.0 + (now * 0.8).sin() * 30.0); scene(&pp.with_clip_rect(Rect::from_min_max(pos2(m + 2.0, r.top()), r.max)), r.translate(vec2(sl, 0.0)), 0.0, 1.0, 100.0 - (now * 0.8).sin() * 30.0); }
        8 => { scene(p, r, 0.55, 1.0, 0.0); let s = Rect::from_min_size(r.right_bottom() - vec2(150.0, 90.0) - vec2(10.0, 10.0), vec2(150.0, 90.0)); p.rect_filled(s.expand(3.0), 4.0, Color32::WHITE); scene(p, s, 0.95, 0.4, (now * 2.0).sin() * 20.0); }
        9 => { scene(p, r, 0.6, 1.0, 0.0); let x = r.left() + tri * r.width(); for j in 0..12 { pp.circle_filled(pos2(x, r.top() + 30.0), 160.0 - j as f32 * 12.0, Color32::from_rgba_unmultiplied(255, 140, 40, 10)); } }
        10 => { scene(p, r, 0.1, 1.0, 0.0); let s = (now * 24.0) as u32; for j in 0..900 { let x = rng(s * 1000 + j); let y = rng(s * 7777 + j * 3); pp.rect_filled(Rect::from_min_size(pos2(r.left() + x * r.width(), r.top() + y * r.height()), vec2(2.0, 2.0)), 0.0, if j % 2 == 0 { Color32::from_white_alpha(60) } else { Color32::from_black_alpha(80) }); } }
        11 => { scene(p, r, 0.1, 1.0, 0.0); let str = 0.4 + tri * 0.6; for j in 0..30 { let rr = r.width() * 0.25 + j as f32 * 8.0; pp.circle_stroke(c, rr, Stroke::new(9.0, Color32::from_black_alpha((j as f32 * 7.0 * str).min(255.0) as u8))); } }
        12 => { let o = 2.0 + tri * 6.0; for (d, col) in [(-o, Color32::from_rgba_unmultiplied(255, 0, 0, 160)), (o, Color32::from_rgba_unmultiplied(0, 120, 255, 160))] { p.text(c + vec2(d, 0.0), Align2::CENTER_CENTER, "CHROMA 色差", f(56.0), col); } p.text(c, Align2::CENTER_CENTER, "CHROMA 色差", f(56.0), Color32::from_rgba_unmultiplied(255, 255, 255, 200)); txt("越靠近邊緣偏移越大", MUTED); }
        13 => { let o = ((now * 9.0).sin() * 10.0).round(); scene(&p.with_clip_rect(r), r.translate(vec2(-o, 0.0)), 0.0, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(0, 255, 255, 40)); scene(&p.with_clip_rect(Rect::from_min_max(r.min, pos2(r.right(), r.top() + 60.0))), r.translate(vec2(o, 0.0)), 0.5, 1.0, 0.0); }
        14 => { scene(p, r, 0.08, 1.0, 0.0); let bh = r.height() * 0.15 * ease(t / 1.0); pp.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), bh)), 0.0, Color32::BLACK); pp.rect_filled(Rect::from_min_max(pos2(r.left(), r.bottom() - bh), r.max), 0.0, Color32::BLACK); txt("2.39:1 電影比例", MUTED); }
        15 => { let m = r.left() + tri * r.width(); scene(&p.with_clip_rect(Rect::from_min_max(r.min, pos2(m, r.bottom()))), r, 0.55, 1.0, 0.0); let right = p.with_clip_rect(Rect::from_min_max(pos2(m, r.top()), r.max)); scene(&right, r, 0.55, 1.0, 0.0); right.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(255, 120, 20, 70)); right.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), r.height() * 0.5)), 0.0, Color32::from_rgba_unmultiplied(0, 160, 160, 40)); p.line_segment([pos2(m, r.top()), pos2(m, r.bottom())], Stroke::new(2.0, Color32::WHITE)); txt("左：原始　右：Teal & Orange LUT", MUTED); }
        16 => { scene(p, r, 0.6, 1.0, 0.0); let e = ease(t / 0.8); let bar = Rect::from_min_size(pos2(r.left() + 16.0 - (1.0 - e) * 300.0, r.bottom() - 60.0), vec2(250.0, 42.0)); p.with_clip_rect(r).rect_filled(bar, 0.0, ACCENT); p.with_clip_rect(r).text(bar.left_center() + vec2(10.0, 0.0), Align2::LEFT_CENTER, "柯盛達　導演 / Director", f(18.0), Color32::WHITE); }
        17 => { let words = ["MOVE", "動き", "BIG", "動態!"]; let j = ((now * 1.5) as usize) % 4; let lt = (now * 1.5).fract(); let s = 1.0 + (1.0 - ease(lt / 0.3)) * 1.5; let ang = if j.is_multiple_of(2) { -0.1 } else { 0.08 } * (1.0 + (s - 1.0) * 3.0); let g = p.layout_no_wrap(words[j].into(), f(60.0 * s), hsv(j as f32 * 0.2)); let half = g.size() / 2.0; let pos = c - vec2(half.x * ang.cos() - half.y * ang.sin(), half.x * ang.sin() + half.y * ang.cos()); p.add(epaint::TextShape::new(pos, g, Color32::WHITE).with_angle(ang)); }
        18 => { let x = cyc; if x < 0.5 { pp.rect_filled(r, 0.0, STAGE); pp.circle_filled(c, 50.0, Color32::from_rgb(240, 200, 120)); p.text(r.left_top() + vec2(8.0, 8.0), Align2::LEFT_TOP, "SHOT A：盤子", f(16.0), Color32::WHITE); } else { pp.rect_filled(r, 0.0, Color32::from_rgb(10, 10, 40)); pp.circle_filled(c, 50.0, Color32::from_rgb(240, 240, 220)); p.text(r.left_top() + vec2(8.0, 8.0), Align2::LEFT_TOP, "SHOT B：月亮（形狀相同）", f(16.0), Color32::WHITE); } }
        // ---- anime
        19 => { pp.rect_filled(r, 0.0, Color32::WHITE); for j in 0..90 { let an = j as f32 / 90.0 * TAU + rng(j + (now * 12.0) as u32) * 0.05; let r0 = 70.0 + rng(j * 3 + (now * 12.0) as u32) * 40.0; pp.add(Shape::convex_polygon(vec![c + vec2(an.cos(), an.sin()) * r0, c + vec2((an + 0.02).cos(), (an + 0.02).sin()) * 400.0, c + vec2((an - 0.02).cos(), (an - 0.02).sin()) * 400.0], Color32::BLACK, Stroke::NONE)); } p.text(c, Align2::CENTER_CENTER, "!!", f(48.0), PINK); }
        20 => { let on = (now * 1.2).fract() < 0.15; if on { pp.rect_filled(r, 0.0, Color32::BLACK); pp.circle_filled(c, 70.0, Color32::WHITE); for j in 0..16 { let an = j as f32 / 16.0 * TAU; pp.line_segment([c, c + vec2(an.cos(), an.sin()) * 300.0], Stroke::new(4.0, Color32::WHITE)); } pp.circle_filled(c, 40.0, Color32::BLACK); } else { pp.rect_filled(r, 0.0, Color32::from_rgb(60, 70, 120)); pp.circle_filled(c, 30.0, Color32::from_rgb(255, 210, 170)); } }
        21 => { let x = r.left() + 40.0 + tri * (r.width() - 80.0); let v = ((cyc * TAU).cos()).abs(); pp.rect_filled(r, 0.0, Color32::from_rgb(30, 36, 58)); for j in 0..8 { pp.circle_filled(pos2(x - j as f32 * 14.0 * v * (cyc * TAU).cos().signum(), c.y), 26.0 - j as f32 * 2.0, Color32::from_rgba_unmultiplied(255, 200, 120, (220 - j * 25) as u8)); } txt("快速動作時把角色拉長、殘影化", MUTED); }
        22 => { let sh = if (now * 0.8).fract() < 0.3 { vec2(rng((now * 60.0) as u32) - 0.5, rng((now * 60.0) as u32 + 9) - 0.5) * 24.0 } else { Vec2::ZERO }; scene(&pp, r.translate(sh), 0.75, 1.0, 0.0); p.text(c + sh, Align2::CENTER_CENTER, "ドン！", f(48.0), Color32::WHITE); }
        23 => { pp.rect_filled(r, 0.0, Color32::from_rgb(120, 170, 230)); for j in 0..30 { let x = r.left() + (rng(j) * r.width() + now * 30.0 + (now + j as f32).sin() * 20.0).rem_euclid(r.width()); let y = r.top() + (rng(j + 100) * r.height() + now * (30.0 + rng(j + 5) * 40.0)).rem_euclid(r.height()); quad(&pp, pos2(x, y), 6.0, 4.0, now * 2.0 + j as f32, 1.0, Color32::from_rgb(255, 183, 197)); } }
        24 => { let x = (now * 0.7).fract(); let s = if x < 0.2 { ease(x / 0.2) * 1.2 } else if x < 0.3 { 1.2 - (x - 0.2) * 2.0 } else { 1.0 }; pp.rect_filled(r, 0.0, Color32::from_rgb(255, 220, 230)); pp.circle_filled(c, 60.0 * s, Color32::from_rgb(255, 210, 170)); pp.circle_filled(c + vec2(-20.0, -5.0) * s, 6.0 * s, Color32::BLACK); pp.circle_filled(c + vec2(20.0, -5.0) * s, 6.0 * s, Color32::BLACK); pp.text(c + vec2(90.0, -50.0), Align2::CENTER_CENTER, "ポン♪", f(24.0), Color32::from_rgb(230, 80, 120)); }
        25 => { pp.rect_filled(r, 0.0, Color32::from_gray(230)); pp.circle_filled(c, 60.0, Color32::from_rgb(255, 210, 170)); let y = (now * 0.8).fract() * 30.0; let dp = c + vec2(55.0, -55.0 + y); pp.add(Shape::convex_polygon(vec![dp + vec2(0.0, -22.0), dp + vec2(10.0, 0.0), dp + vec2(0.0, 10.0), dp + vec2(-10.0, 0.0)], Color32::from_rgb(120, 190, 255), Stroke::new(2.0, Color32::from_rgb(40, 90, 200)))); }
        26 => { pp.rect_filled(r, 0.0, Color32::from_gray(230)); pp.circle_filled(c, 60.0, Color32::from_rgb(255, 210, 170)); let s = 1.0 + ((now * 8.0).sin() * 0.15); let m = c + vec2(50.0, -50.0); for q in 0..4 { let an = q as f32 * PI / 2.0 + PI / 4.0; let d = vec2(an.cos(), an.sin()); let o = m + d * 10.0 * s; let n = vec2(-d.y, d.x); pp.add(Shape::line(vec![o + n * 8.0 * s, o + d * 6.0 * s, o - n * 8.0 * s], Stroke::new(4.0, PINK))); } }
        27 => { pp.rect_filled(r, 0.0, Color32::from_rgb(30, 20, 60)); for j in 0..6 { let ph = (now * 1.5 + j as f32 * 0.37).fract(); let s = (ph * PI).sin(); let pos = pos2(r.left() + rng(j * 11 + (now * 1.5 + j as f32 * 0.37) as u32) * r.width(), r.top() + rng(j * 7 + (now * 1.5 + j as f32 * 0.37) as u32) * r.height()); pp.add(Shape::convex_polygon(star(pos, 18.0 * s, 4), Color32::from_rgb(255, 250, 200), Stroke::NONE)); } }
        28 => { scene(p, r, 0.7, 1.0, 0.0); let x = (now * 0.6).fract(); let a2 = if x < 0.1 { x / 0.1 } else if x < 0.4 { 1.0 - (x - 0.1) / 0.3 } else { 0.0 }; pp.rect_filled(r, 0.0, Color32::from_white_alpha((a2 * 255.0) as u8)); }
        29 => { let w = a.width(); let pr = [Rect::from_min_size(a.min, vec2(w * 0.55, a.height() * 0.55)), Rect::from_min_max(pos2(a.left() + w * 0.57, a.top()), pos2(a.right(), a.top() + a.height() * 0.55)), Rect::from_min_max(pos2(a.left(), a.top() + a.height() * 0.58), a.max)]; for (j, q) in pr.iter().enumerate() { let e = ease(t / 0.5 - j as f32 * 0.6); if e <= 0.0 { continue; } p.rect_filled(q.shrink(2.0), 0.0, Color32::WHITE.gamma_multiply(e)); p.rect_stroke(q.shrink(2.0), 0.0, Stroke::new(3.0, Color32::BLACK)); p.text(q.center(), Align2::CENTER_CENTER, ["コマ1", "コマ2", "コマ3 ドーン"][j], f(22.0), Color32::BLACK.gamma_multiply(e)); } }
        30 => { let ang = 0.25 * tri - 0.05; let o = p.with_clip_rect(r); o.rect_filled(r, 0.0, Color32::from_rgb(60, 60, 80)); for j in 0..4 { quad(&o, c + vec2((j as f32 - 1.5) * 90.0, 30.0), 25.0, 70.0, ang, 1.0, Color32::from_gray(140)); } quad(&o, c + vec2(0.0, 100.0), 300.0, 20.0, ang, 1.0, STAGE); txt("鏡頭傾斜→不安、緊張感", MUTED); }
        31 => { let o = (now * 120.0) % 200.0; pp.rect_filled(r, 0.0, Color32::from_rgb(150, 200, 255)); for j in -1..4 { let x = r.left() - o + j as f32 * 200.0; pp.rect_filled(Rect::from_min_size(pos2(x, r.bottom() - 80.0), vec2(60.0, 80.0)), 0.0, Color32::from_rgb(90, 140, 90)); pp.rect_filled(Rect::from_min_size(pos2(x + 100.0, r.bottom() - 120.0), vec2(50.0, 120.0)), 0.0, Color32::from_rgb(160, 120, 90)); } pp.circle_filled(pos2(c.x, r.bottom() - 30.0 - ((now * 10.0).sin().abs() * 8.0)), 22.0, Color32::from_rgb(255, 120, 120)); }
        32 => { let x = ease(t / 0.8); pp.rect_filled(r, 0.0, Color32::from_rgb(255, 240, 100)); let s = Rect::from_center_size(c, vec2(r.width(), 60.0 * x)); pp.rect_filled(s, 0.0, Color32::from_rgb(230, 60, 100)); p.text(c, Align2::CENTER_CENTER, "★ アイキャッチ ★  CM の後で", f(24.0 * x.max(0.1)), Color32::WHITE); }
        // ---- film
        33 => { scene(p, r, 0.1, 1.0, 0.0); let rr = (1.0 - tri) * 260.0; pp.circle_stroke(c, rr + 250.0, Stroke::new(500.0, Color32::BLACK)); }
        34 => { scene(p, r, 0.6, 1.0, 0.0); let s = tri * 300.0 + 1.0; fan(&pp, c, &star(c, s, 5), hsv(0.1).gamma_multiply(0.95)); txt("星形中露出下一個場景", MUTED); }
        35 => { scene(p, r, 0.0, 1.0, 0.0); let e = cyc * TAU; sector(&pp, c, 400.0, -PI / 2.0, e, hsv(0.6).gamma_multiply(0.85)); }
        36 => { let focus_near = tri; for (j, (x, s, near)) in [(-90.0, 70.0, true), (90.0, 40.0, false)].iter().enumerate() { let blur = if *near { 1.0 - focus_near } else { focus_near } * 10.0; let col = if j == 0 { Color32::from_rgb(255, 160, 90) } else { ACCENT2 }; for q in 0..6 { p.circle_filled(c + vec2(*x, 0.0), s + blur * q as f32 * 0.6, col.gamma_multiply(if blur < 1.0 { 1.0 } else { 0.18 })); } } txt("焦點在前景 ↔ 背景之間切換", MUTED); }
        37 => { let z = 0.6 + tri * 1.2; pp.rect_filled(r, 0.0, STAGE); for j in 0..10 { let d = (j as f32 + 1.0) * 22.0 * z; pp.rect_stroke(Rect::from_center_size(c, vec2(d * 2.0, d * 1.2)), 0.0, Stroke::new(2.0, Color32::from_gray(150))); } pp.circle_filled(c, 26.0, Color32::from_rgb(255, 200, 160)); txt("主體大小不變、背景透視拉伸（希區考克）", MUTED); }
        38 => { pp.rect_filled(r, 0.0, Color32::from_rgb(10, 10, 25)); for j in 0..25 { let pos = pos2(r.left() + rng(j) * r.width(), r.top() + rng(j + 50) * r.height() + (now * 0.5 + j as f32).sin() * 5.0); pp.circle_filled(pos, 12.0 + rng(j + 9) * 22.0, hsv(rng(j + 3) * 0.2 + 0.05).gamma_multiply(0.35)); } }
        39 => { pp.rect_filled(r, 0.0, Color32::from_rgb(20, 20, 40)); let l = pos2(r.left() + 40.0 + tri * 200.0, r.top() + 40.0); let d = c - l; for j in 0..8 { pp.circle_filled(l, 60.0 - j as f32 * 7.0, Color32::from_rgba_unmultiplied(255, 220, 160, 20)); } for (j, k2) in [0.5, 0.9, 1.3, 1.7].iter().enumerate() { pp.circle_stroke(l + d * *k2 * 2.0, 10.0 + j as f32 * 8.0, Stroke::new(3.0, hsv(j as f32 * 0.2).gamma_multiply(0.5))); } pp.line_segment([l - vec2(200.0, 0.0), l + vec2(200.0, 0.0)], Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 220, 160, 90))); }
        40 => { scene(p, r, 0.6, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_black_alpha(90)); for j in 0..120 { let x = r.left() + rng(j) * (r.width() + 60.0); let y = r.top() + (rng(j + 500) * r.height() + now * 600.0 * (0.7 + rng(j + 3) * 0.6)).rem_euclid(r.height()); pp.line_segment([pos2(x, y), pos2(x - 4.0, y + 14.0)], Stroke::new(1.0, Color32::from_rgba_unmultiplied(200, 220, 255, 140))); } }
        41 => { scene(p, r, 0.3, 1.0, 0.0); for j in 0..(r.height() as i32 / 3) { pp.line_segment([pos2(r.left(), r.top() + j as f32 * 3.0), pos2(r.right(), r.top() + j as f32 * 3.0)], Stroke::new(1.0, Color32::from_black_alpha(90))); } let y = r.top() + ((now * 60.0) % r.height()); pp.rect_filled(Rect::from_min_size(pos2(r.left(), y), vec2(r.width(), 10.0)), 0.0, Color32::from_white_alpha(25)); p.text(r.left_top() + vec2(10.0, 8.0), Align2::LEFT_TOP, "▶ PLAY  SP 0:12:34", FontId::monospace(16.0), Color32::WHITE); }
        42 => { scene(p, r, 0.1, 1.0, 0.0); let burn = (now * 0.5).fract(); if burn > 0.6 { let b = (burn - 0.6) / 0.4; pp.circle_filled(r.right_top() + vec2(-40.0, 40.0), b * 260.0, Color32::from_rgba_unmultiplied(255, 120, 20, 200)); pp.circle_filled(r.right_top() + vec2(-40.0, 40.0), b * 160.0, Color32::from_rgba_unmultiplied(255, 255, 230, 230)); }
            if (now * 5.0).sin() > 0.9 { pp.rect_filled(Rect::from_min_size(pos2(r.left(), c.y), vec2(r.width(), 18.0)), 0.0, Color32::from_rgba_unmultiplied(255, 0, 100, 120)); } }
        43 => { let pq = p.with_clip_rect(a); let lines = ["DIRECTED BY", "柯盛達", "", "MUSIC", "Grok Bot", "", "STARRING", "Rust 🦀", "egui", "", "THE END 劇終"]; for (j, l) in lines.iter().enumerate() { let y = a.bottom() - ((now * 40.0) % (a.height() + 400.0)) + j as f32 * 36.0; pq.text(pos2(c.x, y), Align2::CENTER_CENTER, *l, f(if j % 3 == 0 { 16.0 } else { 26.0 }), Color32::WHITE); } }
        44 => { let n = 8 - ((now % 8.0) as i32); let ph = now.fract(); pp.rect_filled(r, 0.0, Color32::from_gray(170)); pp.circle_stroke(c, 80.0, Stroke::new(3.0, Color32::WHITE)); pp.circle_stroke(c, 64.0, Stroke::new(2.0, Color32::WHITE)); pp.line_segment([pos2(r.left(), c.y), pos2(r.right(), c.y)], Stroke::new(2.0, Color32::BLACK)); pp.line_segment([pos2(c.x, r.top()), pos2(c.x, r.bottom())], Stroke::new(2.0, Color32::BLACK)); sector(&pp, c, 300.0, -PI / 2.0, ph * TAU, Color32::from_black_alpha(70)); p.text(c, Align2::CENTER_CENTER, n.to_string(), f(90.0), Color32::BLACK); }
        _ => { let fl = 0.8 + rng((now * 24.0) as u32) * 0.2; scene(p, r, 0.1, 1.0, 0.0); pp.rect_filled(r, 0.0, Color32::from_rgba_unmultiplied(110, 80, 40, 120)); pp.rect_filled(r, 0.0, Color32::from_black_alpha(((1.0 - fl) * 600.0) as u8)); let fr = (now * 24.0) as u32; for j in 0..3 { if rng(fr + j * 13) > 0.5 { let x = r.left() + rng(fr * 3 + j) * r.width(); pp.line_segment([pos2(x, r.top()), pos2(x + 3.0, r.bottom())], Stroke::new(1.0, Color32::from_white_alpha(120))); } } if rng(fr + 999) > 0.8 { pp.circle_filled(pos2(r.left() + rng(fr) * r.width(), r.top() + rng(fr + 1) * r.height()), 4.0, Color32::BLACK); } }
    }
}
