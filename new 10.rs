
    entity_get_field("row_width_scale",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().row_width_scale)
    });
    entity_get_field("col_height_scale",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().col_height_scale)
    });

    // entity_set_field_mut2("row_width_scale",lib_scope,|entity,val,world|{
    //     world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().row_width_scale=val;
    // },|val|script_value_to_float(val));


    entity_set_field_mut("row_width_scale",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().row_width_scale=script_value_to_float(val)?; Ok(())
    });
    entity_set_field_mut("col_height_scale",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().col_height_scale=script_value_to_float(val)?; Ok(())
    });
	

    //
    entity_get_field("padding_left",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.left)
    });
    entity_get_field("padding_right",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.right)
    });
    entity_get_field("padding_top",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.top)
    });
    entity_get_field("padding_bottom",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.bottom)
    });

    entity_set_field_mut("padding_left",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.left=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("padding_right",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.right=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("padding_top",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.top=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("padding_bottom",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.bottom=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("margin_left",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.left)
    });
    entity_get_field("margin_right",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.right)
    });
    entity_get_field("margin_top",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.top)
    });
    entity_get_field("margin_bottom",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.bottom)
    });

    entity_set_field_mut("margin_left",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.left=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("margin_right",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.right=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("margin_top",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.top=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("margin_bottom",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.bottom=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("border_left",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.left)
    });
    entity_get_field("border_right",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.right)
    });
    entity_get_field("border_top",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.top)
    });
    entity_get_field("border_bottom",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.bottom)
    });

    entity_set_field_mut("border_left",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.left=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("border_right",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.right=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("border_top",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.top=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("border_bottom",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.bottom=script_value_to_uival(val)?; Ok(())
    });
	

    entity_get_field("hgap",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().hgap)
    });
    entity_get_field("vgap",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().vgap)
    });

    entity_set_field_mut("hgap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().hgap=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vgap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().vgap=script_value_to_uival(val)?; Ok(())
    });
	

    entity_get_field("hexpand",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().hexpand)
    });
    entity_get_field("vexpand",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().vexpand)
    });

    entity_set_field_mut("hexpand",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().hexpand=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vexpand",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().vexpand=script_value_to_uival(val)?; Ok(())
    });
	
    entity_get_field("hfill",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().hfill)
    });
    entity_get_field("vfill",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().vfill)
    });

    entity_set_field_mut("hfill",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().hfill=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vfill",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().vfill=script_value_to_uival(val)?; Ok(())
    });
	

    entity_get_field("hscroll",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().hscroll)
    });
    entity_get_field("vscroll",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().vscroll)
    });

    entity_set_field_mut("hscroll",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().hscroll=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vscroll",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().vscroll=script_value_to_uival(val)?; Ok(())
    });
	

    entity_get_field("float",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiFloat>().cloned().unwrap_or_default().float)
    });

    entity_set_field_mut("float",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFloat>().or_default().get_mut().float=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("disable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiDisable>().cloned().unwrap_or_default().disable)
    });

    entity_set_field_mut("disable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiDisable>().or_default().get_mut().disable=script_value_to_bool(val)?; Ok(())
    });
	

    entity_get_field("hide",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiHide>().cloned().unwrap_or_default().hide)
    });
    entity_set_field_mut("hide",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiHide>().or_default().get_mut().hide=script_value_to_bool(val)?; Ok(())
    });


    entity_get_field("lock",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiLock>().cloned().unwrap_or_default().lock)
    });
    entity_set_field_mut("lock",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiLock>().or_default().get_mut().lock=script_value_to_bool(val)?; Ok(())
    });
	



    entity_get_field("span",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiSpan>().cloned().unwrap_or_default().span)
    });
    entity_set_field_mut("span",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSpan>().or_default().get_mut().span=script_value_to_uint(val)?; Ok(())
    });
	

    entity_get_field("halign",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().halign)
    });
    entity_get_field("valign",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().valign)
    });

    entity_set_field_mut("halign",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().halign=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("valign",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().valign=script_value_to_uival(val)?; Ok(())
    });
	
    entity_get_field("width",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().width)
    });
    entity_get_field("height",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().height)
    });

    entity_set_field_mut("width",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().width=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("height",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().height=script_value_to_uival(val)?; Ok(())
    });
	


    entity_get_field("hoverable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiHoverable>().cloned().unwrap_or_default().enable)
    });

    entity_set_field_mut("hoverable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiHoverable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
	

    entity_get_field("pressable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().enable)
    });
    entity_get_field("press_always",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().always)
    });
	


    entity_get_field("press_physical",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().physical)
    });

    entity_set_field_mut("pressable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("press_always",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().always=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("press_physical",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().physical=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("draggable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiDraggable>().cloned().unwrap_or_default().enable)
    });

    entity_set_field_mut("draggable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiDraggable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
	

    entity_get_field("selectable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().enable)
    });
    entity_get_field("selected",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().selected)
    });
	

    entity_get_field("select_group",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().group)
    });

    entity_set_field_mut("selectable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("selected",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().selected=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("select_group",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().group=script_value_to_string(val)?; Ok(())
    });

    entity_get_field("focusable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().enable)
    });
    entity_get_field("focused",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().focused)
    });
    entity_get_field("focus_group",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().group)
    });
	

    entity_set_field_mut("focusable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focused",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().focused=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_group",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().group=script_value_to_int(val)?; Ok(())
    });
	

    entity_get_field("focus_tab_exit",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().tab_exit)
    });
    entity_get_field("focus_hdir_exit",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_exit)
    });
    entity_get_field("focus_vdir_exit",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_exit)
    });
	

    entity_set_field_mut("focus_tab_exit",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().tab_exit=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_hdir_exit",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_exit=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_vdir_exit",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_exit=script_value_to_bool(val)?; Ok(())
    });
	

    entity_get_field("focus_hdir_wrap",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_wrap)
    });
    entity_get_field("focus_vdir_wrap",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_wrap)
    });
    entity_set_field_mut("focus_hdir_wrap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_wrap=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_vdir_wrap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_wrap=script_value_to_bool(val)?; Ok(())
    });
	

    entity_get_field("focus_hdir_press",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_press)
    });
    entity_get_field("focus_vdir_press",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_press)
    });

    entity_set_field_mut("focus_hdir_press",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_press=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_vdir_press",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_press=script_value_to_bool(val)?; Ok(())
    });
	
	
    entity_get_field("color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.back_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut("color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().back_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
	
    entity_get_field("padding_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.padding_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_get_field("border_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.border_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_get_field("margin_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.margin_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
	
	 entity_get_field("cell_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.cell_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });


    entity_set_field_mut("padding_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().padding_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("border_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().border_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("margin_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().margin_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("cell_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().cell_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
	

    entity_get_field("text_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.text_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut("text_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().text_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
	
	entity_get_field("image_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().color)
    });

    entity_set_field_mut("image_color",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().color=script_value_to_col(val)?; Ok(())
    });
	

    entity_get_field("image_width",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().width_scale)
    });
    entity_get_field("image_height",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().height_scale)
    });
    entity_set_field_mut("image_width",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().width_scale=script_value_to_float(val)?; Ok(())
    });
    entity_set_field_mut("image_height",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().height_scale=script_value_to_float(val)?; Ok(())
    });
	
	

    entity_get_field("text",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().value)
    });
    entity_get_field("font_size",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().font_size)
    });
    entity_set_field_mut("text",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.value=script_value_to_string(val)?;
        c.update=true;
        Ok(())
    });
	

    entity_set_field_mut("font_size",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.font_size=script_value_to_float(val)?;
        c.update=true;
        Ok(())
    });
	

    entity_get_field("text_hlen",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().hlen)
    });
    entity_get_field("text_vlen",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().vlen)
    });
    entity_set_field_mut("text_hlen",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.hlen=script_value_to_uint(val)?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("text_vlen",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.vlen=script_value_to_uint(val)?;
        c.update=true;
        Ok(())
    });
	
        // let v=v.get_string().and_then(|v|UiTextVAlign::from_str(v.as_str()).ok()).ok_or_else(||MachineError::method("expected halign"))?;

    entity_get_field("text_halign",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().halign.to_string())
    });
    entity_get_field("text_valign",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().valign.to_string())
    });
    entity_set_field_mut("text_halign",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.halign=val.get_parse().ok_or_else(||MachineError::method("expected halign"))?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("text_valign",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.valign=val.get_parse().ok_or_else(||MachineError::method("expected valign"))?;
        c.update=true;
        Ok(())
    });
	
fn node_get_field(mut context:FuncContext<World>) -> Result<Value,MachineError> {
    let entity_val=context.param(0);
    let entity:Entity = entity_val.as_custom().data_copy()?;
    let field=context.param(1).as_string();

    // let world=context.core();
    // let world=context.core();
    let world=context.core_mut();

    Ok(match field.as_str() {
        "row_width_scale" => Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().row_width_scale)
        "col_height_scale" => Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().col_height_scale)

        "padding_left" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.left)
        "padding_right" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.right)
        "padding_top" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.top)
        "padding_bottom" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.bottom)
        "border_left" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.left)
        "border_right" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.right)
        "border_top" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.top)
        "border_bottom" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.bottom)
        "margin_left" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.left)
        "margin_right" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.right)
        "margin_top" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.top)
        "margin_bottom" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.bottom)

        "hgap" => uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().hgap)
        "vgap" => uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().vgap)

        "hexpand" => uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().hexpand)
        "vexpand" => uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().vexpand)

        "hfill" => uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().hfill)
        "vfill" => uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().vfill)

        "hscroll" => uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().hscroll)
        "vscroll" => uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().vscroll)

        "float" => Value::float(world.entity(entity).get::<UiFloat>().cloned().unwrap_or_default().float)

        "disable" => Value::bool(world.entity(entity).get::<UiDisable>().cloned().unwrap_or_default().disable)

        "hide" => Value::bool(world.entity(entity).get::<UiHide>().cloned().unwrap_or_default().hide)

        "lock" => Value::bool(world.entity(entity).get::<UiLock>().cloned().unwrap_or_default().lock)

        "span" => Value::int(world.entity(entity).get::<UiSpan>().cloned().unwrap_or_default().span)

        "halign" => uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().halign)
        "valign" => uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().valign)

        "width" => uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().width)
        "height" => uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().height)

        "hoverable" => Value::bool(world.entity(entity).get::<UiHoverable>().cloned().unwrap_or_default().enable)

        "pressable" => Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().enable)
        "press_always" => Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().always)
        "press_physical" => Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().physical)

        "draggable" => Value::bool(world.entity(entity).get::<UiDraggable>().cloned().unwrap_or_default().enable)

        "selectable" => Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().enable)
        "selected" => Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().selected)
        "select_group" => Value::string(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().group)

        "focusable" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().enable)
        "focused" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().focused)
        "focus_group" => Value::int(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().group)
        "focus_tab_exit" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().tab_exit)
        "focus_hdir_exit" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_exit)
        "focus_vdir_exit" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_exit)
        "focus_hdir_wrap" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_wrap)
        "focus_vdir_wrap" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_wrap)
        "focus_hdir_press" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_press)
        "focus_vdir_press" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_press)

        "color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.back_color.get(&None)).cloned().unwrap_or(Color::NONE))
        "padding_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.padding_color.get(&None)).cloned().unwrap_or(Color::NONE))
        "border_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.back_color.get(&None)).cloned().unwrap_or(Color::NONE))
        "margin_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.margin_color.get(&None)).cloned().unwrap_or(Color::NONE))
        "cell_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.cell_color.get(&None)).cloned().unwrap_or(Color::NONE))
        "text_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.text_color.get(&None)).cloned().unwrap_or(Color::NONE))

        "image_color" => col_to_script_value(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().color)
        "image_width" => Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().width_scale)
        "image_height" => Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().height_scale)

        "text" => Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().value)
        "font_size" => Value::float(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().font_size)
        "text_hlen" => Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().hlen)
        "text_vlen" => Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().vlen)
        "text_halign" => Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().halign.to_string())
        "text_valign" => Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().valign.to_string())

        "parent" => {
            // world.entity(entity).get::<ChildOf>().map(|parent|Value::custom_unmanaged(parent.parent())).unwrap_or(Value::Nil)

            if let Some(parent_entity)=world.entity(entity).get::<ChildOf>().map(|parent|parent.parent()) {
                self_entity_from_world(world,parent_entity)
            } else {
                Value::Nil
            }

        }
        // "children"   => world.entity(entity).get::<Children>().map(|children|children.iter())

        "env" => {
            // let entity_val = self_entity_from_world(world, entity);
            // let name=context.param(1).get_string().unwrap();
            // let name=name.clone();
            Value::custom_unmanaged(Env{ entity:entity_val,  })
        }
        "scaling" => {
            world.entity(entity).get::<UiRoot>().map(|c|Value::float(c.scaling.min(0.0))).unwrap_or_default()
        }
        _ => Value::Nil
    })
}

fn node_set_field(mut context:FuncContext<World>) -> Result<Value,MachineError> {
    let entity:Entity = context.param(0).as_custom().data_copy()?;
    let field=context.param(1).as_string();
    let val=context.param(2);

    // let (world,asset_server)=context.get_core_mut();
    let world=context.core_mut();
    let asset_server=world.resource::<AssetServer>();

    match field.as_str() {
        "row_width_scale" => {
            world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().row_width_scale=script_value_to_float(val)?;
        }
        "col_height_scale" => {
            world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().col_height_scale=script_value_to_float(val)?;
        }

        "padding_left" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.left=script_value_to_uival(val)?;
        }
        "padding_right" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.right=script_value_to_uival(val)?;
        }
        "padding_top" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.top=script_value_to_uival(val)?;
        }
        "padding_bottom" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.bottom=script_value_to_uival(val)?;
        }
        "border_left" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.left=script_value_to_uival(val)?;
        }
        "border_right" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.right=script_value_to_uival(val)?;
        }
        "border_top" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.top=script_value_to_uival(val)?;
        }
        "border_bottom" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.bottom=script_value_to_uival(val)?;
        }
        "margin_left" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.left=script_value_to_uival(val)?;
        }
        "margin_right" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.right=script_value_to_uival(val)?;
        }
        "margin_top" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.top=script_value_to_uival(val)?;
        }
        "margin_bottom" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.bottom=script_value_to_uival(val)?;
        }

        "hgap" => {
            world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().hgap=script_value_to_uival(val)?;
        }
        "vgap" => {
            world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().vgap=script_value_to_uival(val)?;
        }

        "hexpand" => {
            world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().hexpand=script_value_to_uival(val)?;
        }
        "vexpand" => {
            world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().vexpand=script_value_to_uival(val)?;
        }

        "hfill" => {
            world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().hfill=script_value_to_uival(val)?;
        }
        "vfill" => {
            world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().vfill=script_value_to_uival(val)?;
        }

        "hscroll" => {
            world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().hscroll=script_value_to_uival(val)?;
        }
        "vscroll" => {
            world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().vscroll=script_value_to_uival(val)?;
        }

        "float" => {
            world.entity_mut(entity).entry::<UiFloat>().or_default().get_mut().float=script_value_to_bool(val)?;
        }

        "disable" => {
            world.entity_mut(entity).entry::<UiDisable>().or_default().get_mut().disable=script_value_to_bool(val)?;
        }

        "hide" => {
            world.entity_mut(entity).entry::<UiHide>().or_default().get_mut().hide=script_value_to_bool(val)?;
        }

        "lock" => {
            world.entity_mut(entity).entry::<UiLock>().or_default().get_mut().lock=script_value_to_bool(val)?;
        }

        "span" => {
            world.entity_mut(entity).entry::<UiSpan>().or_default().get_mut().span=script_value_to_uint(val)?;
        }

        "halign" => {
            world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().halign=script_value_to_uival(val)?;
        }
        "valign" => {
            world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().valign=script_value_to_uival(val)?;
        }

        "width" => {
            world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().width=script_value_to_uival(val)?;
        }
        "height" => {
            world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().height=script_value_to_uival(val)?;
        }

        "hoverable" => {
            world.entity_mut(entity).entry::<UiHoverable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }

        "pressable" => {
            world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }
        "press_always" => {
            world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().always=script_value_to_bool(val)?;
        }
        "press_physical" => {
            world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().physical=script_value_to_bool(val)?;
        }

        "draggable" => {
            world.entity_mut(entity).entry::<UiDraggable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }

        "selectable" => {
            world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }
        "selected" => {
            world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().selected=script_value_to_bool(val)?;
        }
        "select_group" => {
            world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().group=script_value_to_string(val)?;
        }

        "focusable" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }
        "focused" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().focused=script_value_to_bool(val)?;
        }
        "focus_group" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().group=script_value_to_int(val)?;
        }
        "focus_tab_exit" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().tab_exit=script_value_to_bool(val)?;
        }
        "focus_hdir_exit" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_exit=script_value_to_bool(val)?;
        }
        "focus_vdir_exit" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_exit=script_value_to_bool(val)?;
        }
        "focus_hdir_wrap" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_wrap=script_value_to_bool(val)?;
        }
        "focus_vdir_wrap" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_wrap=script_value_to_bool(val)?;
        }
        "focus_hdir_press" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_press=script_value_to_bool(val)?;
        }
        "focus_vdir_press" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_press=script_value_to_bool(val)?;
        }

        "color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().back_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "padding_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().padding_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "border_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().border_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "margin_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().margin_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "cell_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().cell_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "text_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().text_color.entry(None).or_default()=script_value_to_col(val)?;
        }

        "image" => {
            let handle=asset_server.load(PathBuf::from(script_value_to_string(val)?));
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiImage>().or_default();
            let mut c=c.get_mut();
            c.handle=handle;
            e.entry::<UiInnerSize>().or_default();
        }
        "image_color" => {
            world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().color=script_value_to_col(val)?;
        }
        "image_width" => {
            world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().width_scale=script_value_to_float(val)?;
        }
        "image_height" => {
            world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().height_scale=script_value_to_float(val)?;
        }

        "text" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.value=script_value_to_string(val)?;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "font" => {
            let handle=asset_server.load(PathBuf::from(script_value_to_string(val)?));

            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.font=handle;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "font_size" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.font_size=script_value_to_float(val)?;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_hlen" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.hlen=script_value_to_uint(val)?;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_vlen" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.vlen=script_value_to_uint(val)?;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_halign" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.halign=val.get_parse().ok_or_else(||MachineError::method("expected halign"))?;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_valign" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            let mut c=c.get_mut();
            c.valign=val.get_parse().ok_or_else(||MachineError::method("expected valign"))?;
            c.update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "parent" => {
            //let parent=val.as_custom().data_copy()?;
            // let mut e=world.entity_mut(entity);
            // e.set_parent(parent);

            //do nothing
        }
        "env" => {
            //do nothing when tried to be set
            //  could check if the Env's entity is same as this entity, and return an err if not
        }

        "scaling" => {
            if let Some(mut c)=world.entity_mut(entity).get_mut::<UiRoot>() {
                c.scaling = script_value_to_float(val)?.min(0.0)
            }
        }
        _ => {
            return Err(MachineError::method("invalid field"));
        }
    }

    Ok(Value::Void)
}


 //
    lib_scope.field_named("padding_left",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.left))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("padding_right",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.right))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("padding_top",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.top))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("padding_bottom",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.bottom))
    }).custom_ref::<Entity>().end();

    //
    lib_scope.field_named("margin_left",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.left))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("margin_right",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.right))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("margin_top",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.top))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("margin_bottom",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.bottom))
    }).custom_ref::<Entity>().end();

    //
    lib_scope.field_named("border_left",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.left))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("border_right",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.right))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("border_top",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.top))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("border_bottom",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.bottom))
    }).custom_ref::<Entity>().end();

    //
    lib_scope.field_named("hgap",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().hgap))
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("vgap",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().vgap))
    }).custom_ref::<Entity>().end();


    //
    lib_scope.field_named("hexpand",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().hgap))
    }).custom_ref::<Entity>().end();